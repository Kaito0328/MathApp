use std::fmt::Display;
use std::fs::File;
use std::io::{Result, Write};

// --- Public API ---
#[derive(Clone, Debug)]
pub enum BlockExpr {
    Atom(String),
    Series(Box<BlockExpr>, Box<BlockExpr>),
    Parallel(Box<BlockExpr>, Box<BlockExpr>),
    Feedback {
        forward: Box<BlockExpr>,
        feedback: Option<Box<BlockExpr>>, // None => unity
        negative: bool,
    },
}

#[derive(Clone, Debug)]
pub struct DiagramTf {
    pub expr: BlockExpr,
}

impl DiagramTf {
    pub fn atom_label(label: impl Into<String>) -> Self {
        Self {
            expr: BlockExpr::Atom(label.into()),
        }
    }
    pub fn atom<T: Display>(t: &T) -> Self {
        Self {
            expr: BlockExpr::Atom(format!("{t}")),
        }
    }
    pub fn series(self, other: DiagramTf) -> Self {
        Self {
            expr: BlockExpr::Series(Box::new(self.expr), Box::new(other.expr)),
        }
    }
    pub fn parallel(self, other: DiagramTf) -> Self {
        Self {
            expr: BlockExpr::Parallel(Box::new(self.expr), Box::new(other.expr)),
        }
    }
    pub fn feedback(self, h: Option<DiagramTf>, negative: bool) -> Self {
        let fb: Option<Box<BlockExpr>> = h.map(|d| Box::new(d.expr));
        Self {
            expr: BlockExpr::Feedback {
                forward: Box::new(self.expr),
                feedback: fb,
                negative,
            },
        }
    }

    pub fn to_svg(&self, path: &str, width: u32, height: u32) -> Result<()> {
        let mut f = File::create(path)?;
        writeln!(f, "<svg xmlns='http://www.w3.org/2000/svg' width='{width}' height='{height}' viewBox='0 0 {width} {height}'>")?;
        writeln!(
            f,
            "<defs><style>
            .bg {{ fill:#ffffff; }}
            .blk {{ fill:#ffffff; stroke:#222; stroke-width:1.5; }}
            .sum {{ fill:#fff; stroke:#222; stroke-width:1.5; }}
            .split {{ fill:#222; }}
            .wire {{ stroke:#222; stroke-width:1.5; fill:none; }}
            .arrow {{ stroke:#222; stroke-width:1.5; fill:none; marker-end:url(#arrow); }}
            .text {{ font-family: 'DejaVu Sans', Arial, sans-serif; font-size:12px; fill:#222; }}
        </style>
        <marker id='arrow' markerWidth='10' markerHeight='8' refX='10' refY='4' orient='auto'>
          <path d='M 0 0 L 10 4 L 0 8 z' fill='#222'/>
        </marker></defs>"
        )?;
        writeln!(
            f,
            "<rect class='bg' x='0' y='0' width='{width}' height='{height}' />"
        )?;

        let margin = 20.0;
        let area_w = (width as f64) - margin * 2.0;
        let area_h = (height as f64) - margin * 2.0;
        let layout = layout_expr(&self.expr);
        let sx = if layout.w > 0.0 {
            area_w / layout.w
        } else {
            1.0
        };
        let sy = if layout.h > 0.0 {
            area_h / layout.h
        } else {
            1.0
        };
        let s = sx.min(sy);
        // translate to center
        let tx = margin + (area_w - layout.w * s) / 2.0;
        let ty = margin + (area_h - layout.h * s) / 2.0;

        // Render primitives
        for p in &layout.prims {
            match p {
                Prim::Rect { x, y, w, h, label } => {
                    let (x, y, w, h) = (tx + x * s, ty + y * s, w * s, h * s);
                    writeln!(f, "<rect class='blk' x='{x:.1}' y='{y:.1}' width='{w:.1}' height='{h:.1}' rx='6' ry='6' />")?;
                    let cx = x + w / 2.0;
                    let cy = y + h / 2.0 - 6.0; // raise text a bit
                    writeln!(
                        f,
                        "<text class='text' x='{cx:.1}' y='{cy:.1}' text-anchor='middle'>{}</text>",
                        xml_escape(label)
                    )?;
                }
                Prim::Sum {
                    cx,
                    cy,
                    r,
                    upper_plus,
                    lower_plus,
                } => {
                    let (cx, cy, r) = (tx + cx * s, ty + cy * s, r * s);
                    writeln!(
                        f,
                        "<circle class='sum' cx='{cx:.1}' cy='{cy:.1}' r='{r:.1}' />"
                    )?;
                    // put signs outside circle for readability
                    let y_up = cy - r - 4.0;
                    let y_down = cy + r + 12.0;
                    let up = if *upper_plus { "+" } else { "-" };
                    let down = if *lower_plus { "+" } else { "-" };
                    writeln!(f, "<text class='text' x='{cx:.1}' y='{y_up:.1}' text-anchor='middle'>{up}</text>")?;
                    writeln!(f, "<text class='text' x='{cx:.1}' y='{y_down:.1}' text-anchor='middle'>{down}</text>")?;
                }
                Prim::Dot { cx, cy, r } => {
                    let (cx, cy, r) = (tx + cx * s, ty + cy * s, r * s);
                    writeln!(
                        f,
                        "<circle class='split' cx='{cx:.1}' cy='{cy:.1}' r='{r:.1}' />"
                    )?;
                }
                Prim::Line {
                    x1,
                    y1,
                    x2,
                    y2,
                    arrow,
                } => {
                    let (x1, y1, x2, y2) = (tx + x1 * s, ty + y1 * s, tx + x2 * s, ty + y2 * s);
                    if *arrow {
                        writeln!(f, "<line class='arrow' x1='{x1:.1}' y1='{y1:.1}' x2='{x2:.1}' y2='{y2:.1}' />")?;
                    } else {
                        writeln!(f, "<line class='wire' x1='{x1:.1}' y1='{y1:.1}' x2='{x2:.1}' y2='{y2:.1}' />")?;
                    }
                }
                Prim::PathV { x, y1, y2 } => {
                    let (x, y1, y2) = (tx + x * s, ty + y1 * s, ty + y2 * s);
                    writeln!(f, "<path class='wire' d='M {x:.1} {y1:.1} V {y2:.1}' />")?;
                }
            }
        }

        writeln!(f, "</svg>")?;
        Ok(())
    }
}

// --- Internal layout / rendering ---

#[derive(Clone, Debug)]
struct Layout {
    w: f64,
    h: f64,
    entry: (f64, f64),
    exit: (f64, f64),
    prims: Vec<Prim>,
}

#[derive(Clone, Debug)]
enum Prim {
    Rect {
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        label: String,
    },
    Sum {
        cx: f64,
        cy: f64,
        r: f64,
        upper_plus: bool,
        lower_plus: bool,
    },
    Dot {
        cx: f64,
        cy: f64,
        r: f64,
    },
    Line {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        arrow: bool,
    },
    PathV {
        x: f64,
        y1: f64,
        y2: f64,
    },
}

fn xml_escape(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '&' => "&amp;".to_string(),
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&apos;".to_string(),
            _ => c.to_string(),
        })
        .collect()
}

const BW_MIN: f64 = 120.0; // block width min
const BH: f64 = 46.0; // block height
const SX: f64 = 40.0; // horizontal spacing
const SY: f64 = 50.0; // vertical spacing
const SUM_R: f64 = 12.0;

fn block_size(label: &str) -> (f64, f64) {
    // crude estimate by character count
    let w = (label.chars().count() as f64) * 7.0 + 24.0;
    (w.max(BW_MIN), BH)
}

fn layout_expr(expr: &BlockExpr) -> Layout {
    match expr {
        BlockExpr::Atom(label) => {
            let (bw, bh) = block_size(label);
            let prims = vec![Prim::Rect {
                x: 0.0,
                y: 0.0,
                w: bw,
                h: bh,
                label: label.clone(),
            }];
            Layout {
                w: bw,
                h: bh,
                entry: (0.0, bh / 2.0),
                exit: (bw, bh / 2.0),
                prims,
            }
        }
        BlockExpr::Series(a, b) => {
            let la = layout_expr(a);
            let lb = layout_expr(b);
            let h = la.h.max(lb.h);
            let ax = 0.0;
            let ay = (h - la.h) / 2.0;
            let bx = la.w + SX;
            let by = (h - lb.h) / 2.0;
            let w = bx + lb.w;
            let mut prims = Vec::new();
            // place a
            for p in la.prims {
                prims.push(translate(p, ax, ay));
            }
            // wire a -> b
            prims.push(Prim::Line {
                x1: la.exit.0 + ax,
                y1: la.exit.1 + ay,
                x2: bx + lb.entry.0,
                y2: by + lb.entry.1,
                arrow: true,
            });
            // place b
            for p in lb.prims {
                prims.push(translate(p, bx, by));
            }
            Layout {
                w,
                h,
                entry: (la.entry.0 + ax, la.entry.1 + ay),
                exit: (bx + lb.exit.0, by + lb.exit.1),
                prims,
            }
        }
        BlockExpr::Parallel(a, b) => {
            let la = layout_expr(a);
            let lb = layout_expr(b);
            let w = la.w.max(lb.w) + 2.0 * SX;
            let h = la.h + lb.h + SY + SUM_R * 2.0;
            let mid_y = h / 2.0;
            let split_x = SX * 0.5;
            let sum_x = w - SX * 0.5;
            let top_y = mid_y - (SY / 2.0 + lb.h / 2.0 + la.h / 2.0) + la.h / 2.0; // center
            let bot_y = top_y + la.h + SY;
            let top_x = SX;
            let bot_x = SX;

            let mut prims = vec![
                Prim::Dot {
                    cx: split_x,
                    cy: mid_y,
                    r: 3.0,
                },
                Prim::Line {
                    x1: 0.0,
                    y1: mid_y,
                    x2: split_x,
                    y2: mid_y,
                    arrow: true,
                },
                Prim::Line {
                    x1: split_x,
                    y1: mid_y,
                    x2: top_x,
                    y2: top_y + la.entry.1,
                    arrow: false,
                },
                Prim::Line {
                    x1: split_x,
                    y1: mid_y,
                    x2: bot_x,
                    y2: bot_y + lb.entry.1,
                    arrow: false,
                },
            ];

            // place top and bottom blocks
            for p in la.prims {
                prims.push(translate(p, top_x, top_y));
            }
            for p in lb.prims {
                prims.push(translate(p, bot_x, bot_y));
            }

            // sum on right
            prims.push(Prim::Sum {
                cx: sum_x,
                cy: mid_y,
                r: SUM_R,
                upper_plus: true,
                lower_plus: true,
            });
            // wires from block exits to sum
            prims.push(Prim::Line {
                x1: top_x + la.exit.0,
                y1: top_y + la.exit.1,
                x2: sum_x - SUM_R,
                y2: mid_y,
                arrow: true,
            });
            prims.push(Prim::Line {
                x1: bot_x + lb.exit.0,
                y1: bot_y + lb.exit.1,
                x2: sum_x - SUM_R,
                y2: mid_y,
                arrow: true,
            });
            // exit to right
            prims.push(Prim::Line {
                x1: sum_x + SUM_R,
                y1: mid_y,
                x2: w,
                y2: mid_y,
                arrow: true,
            });

            Layout {
                w,
                h,
                entry: (0.0, mid_y),
                exit: (w, mid_y),
                prims,
            }
        }
        BlockExpr::Feedback {
            forward,
            feedback,
            negative,
        } => {
            let lf = layout_expr(forward);
            let loop_drop = 80.0;
            let w = lf.w + 2.0 * SX;
            let h = lf.h + loop_drop + SUM_R * 2.0;
            let mid_y = lf.h / 2.0 + SUM_R;
            let sum_x = SX * 0.7;
            let block_x = sum_x + SUM_R + SX * 0.3;
            let block_y = SUM_R;
            let out_x = block_x + lf.w + SX * 0.5;
            let mut prims = Vec::new();
            // sum
            prims.push(Prim::Sum {
                cx: sum_x,
                cy: mid_y,
                r: SUM_R,
                upper_plus: true,
                lower_plus: !*negative,
            });
            // entry
            prims.push(Prim::Line {
                x1: 0.0,
                y1: mid_y,
                x2: sum_x - SUM_R,
                y2: mid_y,
                arrow: true,
            });
            // forward
            for p in lf.prims {
                prims.push(translate(p, block_x, block_y));
            }
            // wires: sum -> forward -> out
            prims.push(Prim::Line {
                x1: sum_x + SUM_R,
                y1: mid_y,
                x2: block_x + lf.entry.0,
                y2: block_y + lf.entry.1,
                arrow: true,
            });
            prims.push(Prim::Line {
                x1: block_x + lf.exit.0,
                y1: block_y + lf.exit.1,
                x2: out_x,
                y2: mid_y,
                arrow: true,
            });

            // feedback path
            let fb_y = mid_y + loop_drop;
            // down from output
            prims.push(Prim::PathV {
                x: out_x,
                y1: mid_y,
                y2: fb_y,
            });
            // to left, possibly through H
            let left_end_x = sum_x;
            if let Some(h) = feedback {
                let lh = layout_expr(h);
                let h_w = lh.w.max(BW_MIN);
                let h_x = (block_x + lf.w + out_x) / 2.0 - h_w / 2.0;
                let h_y = fb_y - lh.h / 2.0;
                // wires under block (left and right)
                prims.push(Prim::Line {
                    x1: out_x,
                    y1: fb_y,
                    x2: h_x,
                    y2: fb_y,
                    arrow: false,
                });
                prims.push(Prim::Line {
                    x1: h_x + h_w,
                    y1: fb_y,
                    x2: sum_x,
                    y2: fb_y,
                    arrow: false,
                });
                // draw block
                for p in lh.prims {
                    prims.push(translate(p, h_x, h_y));
                }
            } else {
                prims.push(Prim::Line {
                    x1: out_x,
                    y1: fb_y,
                    x2: left_end_x,
                    y2: fb_y,
                    arrow: false,
                });
            }
            // up to sum bottom
            prims.push(Prim::PathV {
                x: left_end_x,
                y1: fb_y,
                y2: mid_y + SUM_R,
            });
            prims.push(Prim::Line {
                x1: left_end_x,
                y1: mid_y + SUM_R,
                x2: left_end_x,
                y2: mid_y + 1.0,
                arrow: true,
            });

            Layout {
                w,
                h,
                entry: (0.0, mid_y),
                exit: (out_x, mid_y),
                prims,
            }
        }
    }
}

fn translate(p: Prim, dx: f64, dy: f64) -> Prim {
    match p {
        Prim::Rect { x, y, w, h, label } => Prim::Rect {
            x: x + dx,
            y: y + dy,
            w,
            h,
            label,
        },
        Prim::Sum {
            cx,
            cy,
            r,
            upper_plus,
            lower_plus,
        } => Prim::Sum {
            cx: cx + dx,
            cy: cy + dy,
            r,
            upper_plus,
            lower_plus,
        },
        Prim::Dot { cx, cy, r } => Prim::Dot {
            cx: cx + dx,
            cy: cy + dy,
            r,
        },
        Prim::Line {
            x1,
            y1,
            x2,
            y2,
            arrow,
        } => Prim::Line {
            x1: x1 + dx,
            y1: y1 + dy,
            x2: x2 + dx,
            y2: y2 + dy,
            arrow,
        },
        Prim::PathV { x, y1, y2 } => Prim::PathV {
            x: x + dx,
            y1: y1 + dy,
            y2: y2 + dy,
        },
    }
}
