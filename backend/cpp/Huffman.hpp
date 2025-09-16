#include <bits/stdc++.h>
using namespace std;

#include "code.hpp"

// ハフマン木のノードを表す構造体
typedef struct HuffmanNode_t {
  HuffmanNode_t* parent = nullptr;
  vector<HuffmanNode_t*> children = {};
  bool is_leaf = false;
  char symbol = '\0';
  float pr = 0;

  HuffmanNode_t(int q) { children = vector<HuffmanNode_t*>(q); };
  HuffmanNode_t(char symbol, float pr)
      : is_leaf(true), symbol(symbol), pr(pr) {};
}* HuffmanNode;

// 最小値を取り出せるヒープ構造を扱うクラス
// 要素はハフマン木のノードであり、ノードの確率によってソートする
class HuffmanMinHeap {
 private:
  vector<HuffmanNode> heap;

  // 子のインデックスを引数に取り、子が親よりも小さかった場合に入れ替える。
  // これを上向きに再帰的に行い、ヒープ構造を保つよう調整する
  void modify_up(int child) {
    if (child == 0) return;
    int parent = (child - 1) / 2;

    if (heap[child]->pr < heap[parent]->pr) {
      swap(heap[child], heap[parent]);
      modify_up(parent);
    }
  }

  // 親のインデックスを引数に取り、親が子よりも大きかった場合に、最も小さい子要素と入れ替える
  // これを下向きに再帰的に行い、ヒープ構造を保つよう調整する
  void modify_down(int parent) {
    int left = 2 * parent + 1;
    int right = 2 * parent + 2;
    int smallest = parent;

    if (left < (int)heap.size() && heap[left]->pr < heap[smallest]->pr)
      smallest = left;
    if (right < (int)heap.size() && heap[right]->pr < heap[smallest]->pr)
      smallest = right;

    if (smallest == parent) return;

    swap(heap[parent], heap[smallest]);
    modify_down(smallest);
  }

 public:
  // 新しいノードを挿入する
  // ヒープ構造を保つよう適切な位置に挿入される
  void insert(HuffmanNode value) {
    heap.push_back(value);
    modify_up(heap.size() - 1);
  }

  // 最小値(根)を取り出し、削除する
  // ヒープ構造を保つよう調整がかかる
  HuffmanNode pop_min(void) {
    HuffmanNode min = heap[0];
    heap[0] = heap.back();
    heap.pop_back();
    if (!heap.empty()) modify_down(0);

    return min;
  }
  // ヒープがからかどうかを返す
  bool empty() { return heap.empty(); }

  // ヒープの要素数を返す
  int size() { return (int)heap.size(); }
};

// ハフマン木を作成し、記号に対応する符号語を返すクラス
// 元の数, 記号と発生確率を引数に取り初期化する
class HuffmanTree {
 private:
  vector<Symbol> alphabet;
  SymbolToPr symbol_to_prs;
  SymbolToCodeWord s_to_cws;
  HuffmanMinHeap heap;
  int q;
  HuffmanNode root = nullptr;

  // 記号の発生確率からヒープを作り返す
  HuffmanMinHeap make_heap() {
    HuffmanMinHeap heap;
    HuffmanNode root;
    for (Symbol symbol : alphabet) {
      heap.insert(new HuffmanNode_t(symbol, symbol_to_prs.at(symbol)));
    }

    int N = (int)alphabet.size();
    if (q > 2) {
      for (int i = 0; i < (q - 1) - (N - 1) % (q - 1); i++) {
        heap.insert(new HuffmanNode_t('\0', 0.0));
      }
    }
    return heap;
  }

  // ハフマン木を作成する
  void make_tree() {
    HuffmanMinHeap heap = make_heap();

    while (heap.size() > 1) {
      HuffmanNode mix_node = new HuffmanNode_t(q);
      for (int i = 0; i < q; i++) {
        HuffmanNode min_node = heap.pop_min();
        mix_node->children[i] = min_node;
        mix_node->pr += min_node->pr;
        min_node->parent = mix_node;
      }
      heap.insert(mix_node);
      root = mix_node;
    }
  }

  // 動的確保したノードを再帰的に削除する
  void delete_tree(HuffmanNode node) {
    if (!node) return;
    if (!node->is_leaf) {
      for (int i = 0; i < q; i++) {
        delete_tree(node->children[i]);
      }
    }
    delete node;
  }

  // ノードと符号語を引数に取り、符号語を構築する
  // ノードが葉ならs_to_cwsに符号語を加える
  // ノードが葉でないなら、符号語に対応する番号を加えて、子ノードへ進む
  void build_code(HuffmanNode node, vector<int> cw) {
    if (node->is_leaf) {
      s_to_cws[node->symbol] = cw;
      return;
    }

    for (int i = 0; i < q; i++) {
      if (node->children[i] == nullptr) continue;

      vector<int> add_code = cw;
      add_code.push_back(q - 1 - i);
      build_code(node->children[i], add_code);
    }
  }

 public:
  // 元の数, 記号と発生確率を受け取るコンストラクタ
  HuffmanTree(int _q, SymbolPr symbolPr) {
    q = _q;
    alphabet = symbolPr.alphabet;
    symbol_to_prs = symbolPr.s_to_prs;
  }

  // ハフマン符号語 : 記号に対応する符号語を取り出せるmapを返す
  SymbolToCodeWord huffman_code() {
    make_tree();
    build_code(root, {});
    delete_tree(root);
    return s_to_cws;
  }
};