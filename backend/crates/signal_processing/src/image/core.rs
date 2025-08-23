use image::{DynamicImage, GrayImage, ImageResult, RgbImage};

#[derive(Clone, Debug)]
pub struct Image<P> {
    data: Vec<P>,
    width: usize,
    height: usize,
}

pub trait Pixel: Copy + Default + Send + Sync {}

impl<P: Pixel> Image<P> {
    pub fn new(width: usize, height: usize) -> Self {
        let data = vec![P::default(); width * height];
        Self {
            data,
            width,
            height,
        }
    }

    pub fn from_vec(data: Vec<P>, width: usize, height: usize) -> Self {
        assert_eq!(
            data.len(),
            width * height,
            "data length must be width*height"
        );
        Self {
            data,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<P> {
        if x < self.width && y < self.height {
            Some(self.data[y * self.width + x])
        } else {
            None
        }
    }

    pub fn get_pixel_mut(&mut self, x: usize, y: usize) -> Option<&mut P> {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            Some(&mut self.data[idx])
        } else {
            None
        }
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, value: P) -> Option<()> {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = value;
            Some(())
        } else {
            None
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[P] {
        &self.data
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [P] {
        &mut self.data
    }

    /// (x,y) から行優先（row-major）のインデックスを返す（境界チェックなし）
    #[inline]
    pub fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn crop(&self, x: usize, y: usize, width: usize, height: usize) -> Option<Image<P>> {
        if x + width <= self.width && y + height <= self.height {
            let mut data = Vec::with_capacity(width * height);
            for j in y..y + height {
                let start = j * self.width + x;
                let end = start + width;
                data.extend_from_slice(&self.data[start..end]);
            }
            Some(Image::from_vec(data, width, height))
        } else {
            None
        }
    }

    /// 画像全体を値で塗りつぶし
    pub fn fill(&mut self, value: P) {
        for v in &mut self.data {
            *v = value;
        }
    }

    /// 画素変換（新しい画像を返す）
    pub fn map<Q: Pixel>(&self, mut f: impl FnMut(P) -> Q) -> Image<Q> {
        let mut out = Vec::with_capacity(self.data.len());
        for &p in &self.data {
            out.push(f(p));
        }
        Image {
            data: out,
            width: self.width,
            height: self.height,
        }
    }

    /// 画素生成関数から画像を作る
    pub fn from_fn(width: usize, height: usize, mut f: impl FnMut(usize, usize) -> P) -> Image<P> {
        let mut data = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                data.push(f(x, y));
            }
        }
        Image {
            data,
            width,
            height,
        }
    }

    pub fn pixels(&self) -> impl Iterator<Item = &P> {
        self.data.iter()
    }

    pub fn pixels_mut(&mut self) -> impl Iterator<Item = &mut P> {
        self.data.iter_mut()
    }

    pub fn rows(&self) -> impl Iterator<Item = &[P]> {
        self.data.chunks(self.width)
    }

    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [P]> {
        self.data.chunks_mut(self.width)
    }
}

// Pixel 実装（最低限）
impl Pixel for u8 {}
impl Pixel for f32 {}
impl Pixel for [u8; 3] {}
impl Pixel for [f32; 3] {}

// 基本的な I/O（PNG）
impl Image<u8> {
    /// グレースケール PNG として保存（8bit）
    pub fn save_png(&self, path: &str) -> ImageResult<()> {
        image::GrayImage::from_raw(self.width as u32, self.height as u32, self.data.clone())
            .expect("container size is invalid")
            .save(path)
    }

    /// f32 グレー画像から vmin..vmax で正規化して u8 へ（クランプ）
    pub fn from_f32_normalized(src: &Image<f32>, vmin: f32, vmax: f32) -> Image<u8> {
        let denom = if vmax > vmin { vmax - vmin } else { 1.0 };
        let data: Vec<u8> = src
            .as_slice()
            .iter()
            .map(|&v| (((v - vmin) / denom).clamp(0.0, 1.0) * 255.0).round() as u8)
            .collect();
        Image {
            data,
            width: src.width,
            height: src.height,
        }
    }

    /// GrayImage（imageクレート）への変換（所有を保持）
    pub fn to_gray_image(&self) -> GrayImage {
        GrayImage::from_raw(self.width as u32, self.height as u32, self.data.clone())
            .expect("container size is invalid")
    }

    /// GrayImage からの変換（所有を消費）
    pub fn from_gray_image(img: GrayImage) -> Self {
        let (w, h) = img.dimensions();
        let data = img.into_raw();
        Image {
            data,
            width: w as usize,
            height: h as usize,
        }
    }

    /// GrayImage からの変換（借用をコピー）
    pub fn from_gray_image_ref(img: &GrayImage) -> Self {
        let (w, h) = img.dimensions();
        let data = img.as_raw().clone();
        Image {
            data,
            width: w as usize,
            height: h as usize,
        }
    }

    /// 拡張子から自動判別して読み込み（任意フォーマットをグレースケールへ）
    pub fn load_from_path_as_gray(path: &str) -> ImageResult<Self> {
        let dynimg = image::open(path)?; // DynamicImage
        let g = dynimg.to_luma8();
        Ok(Self::from_gray_image(g))
    }

    /// 拡張子から自動判別して保存（png/jpeg/bmp/gif 等、利用可能な機能に依存）
    pub fn save_to_path(&self, path: &str) -> ImageResult<()> {
        let dynimg = DynamicImage::ImageLuma8(self.to_gray_image());
        dynimg.save(path)
    }
}

impl Image<[u8; 3]> {
    /// RGB PNG として保存（各 8bit）
    // Image<[u8; 3]> の場合 (一度フラットなVec<u8>に変換)
    pub fn save_png(&self, path: &str) -> ImageResult<()> {
        let flat_data: Vec<u8> = self.data.iter().flat_map(|&rgb| rgb).collect();
        image::RgbImage::from_raw(self.width as u32, self.height as u32, flat_data)
            .expect("container size is invalid")
            .save(path)
    }

    /// RgbImage（imageクレート）への変換
    pub fn to_rgb_image(&self) -> RgbImage {
        let flat_data: Vec<u8> = self.data.iter().flat_map(|&rgb| rgb).collect();
        RgbImage::from_raw(self.width as u32, self.height as u32, flat_data)
            .expect("container size is invalid")
    }

    /// RgbImage からの変換（所有を消費）
    pub fn from_rgb_image(img: RgbImage) -> Self {
        let (w, h) = img.dimensions();
        let raw = img.into_raw();
        let mut data = Vec::with_capacity((w * h) as usize);
        for c in raw.chunks_exact(3) {
            data.push([c[0], c[1], c[2]]);
        }
        Image {
            data,
            width: w as usize,
            height: h as usize,
        }
    }

    /// 画像読み込み（RGB化して取り込む）
    pub fn load_from_path_as_rgb(path: &str) -> ImageResult<Self> {
        let dynimg = image::open(path)?;
        let rgb = dynimg.to_rgb8();
        Ok(Self::from_rgb_image(rgb))
    }

    /// 拡張子から自動判別して保存
    pub fn save_to_path(&self, path: &str) -> ImageResult<()> {
        let dynimg = DynamicImage::ImageRgb8(self.to_rgb_image());
        dynimg.save(path)
    }
}
