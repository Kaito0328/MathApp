#ifndef MY_HEADER_HPP
#define MY_HEADER_HPP
#include "code.hpp"
#endif
#include <bits/stdc++.h>
using namespace std;

// ブロックハフマン木のノードを表す構造体
typedef struct BlockHuffmanNode_t {
  BlockHuffmanNode_t* parent = nullptr;
  vector<BlockHuffmanNode_t*> children = {};
  bool is_leaf = false;
  Symbols symbols = "\0";
  float pr = 0;

  BlockHuffmanNode_t(int q) { children = vector<BlockHuffmanNode_t*>(q); };
  BlockHuffmanNode_t(Symbols symbols, float pr)
      : is_leaf(true), symbols(symbols), pr(pr) {};
}* BlockHuffmanNode;

// 最小値を取り出せるヒープ構造を扱うクラス
// 要素はブロックハフマン木のノードであり、ノードの確率によってソートする
class BlockHuffmanMinHeap {
 private:
  vector<BlockHuffmanNode> heap;

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
  void insert(BlockHuffmanNode value) {
    heap.push_back(value);
    modify_up(heap.size() - 1);
  }

  // 最小値(根)を取り出し、削除する
  // ヒープ構造を保つよう調整がかかる
  BlockHuffmanNode pop_min(void) {
    BlockHuffmanNode min = heap[0];
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

// ブロックハフマン木を作成し、ブロック化した記号列に対応する符号語を返すクラス
// 元の数, 記号列と発生確率を引数に取り初期化する
class BlockHuffmanTree {
 private:
  SymbolsPr symbolsPr;
  SymbolsToCodeWord ss_to_cws;
  BlockHuffmanMinHeap heap;
  int q;
  BlockHuffmanNode root = nullptr;

  // 記号列の発生確率からヒープを作り返す
  BlockHuffmanMinHeap make_heap() {
    BlockHuffmanMinHeap heap;
    BlockHuffmanNode root;
    for (Symbols symbols : symbolsPr.block_alphabet) {
      heap.insert(
          new BlockHuffmanNode_t(symbols, symbolsPr.ss_to_prs.at(symbols)));
    }

    int N = (int)symbolsPr.block_alphabet.size();
    if (q > 2) {
      for (int i = 0; i < (q - 1) - (N - 1) % (q - 1); i++) {
        heap.insert(new BlockHuffmanNode_t("\0", 0.0));
      }
    }
    return heap;
  }

  // ブロックハフマン木を作成する
  void make_tree() {
    BlockHuffmanMinHeap heap = make_heap();

    while (heap.size() > 1) {
      BlockHuffmanNode mix_node = new BlockHuffmanNode_t(q);
      for (int i = 0; i < q; i++) {
        BlockHuffmanNode min_node = heap.pop_min();
        mix_node->children[i] = min_node;
        mix_node->pr += min_node->pr;
        min_node->parent = mix_node;
      }
      heap.insert(mix_node);
      root = mix_node;
    }
  }

  // 動的確保したノードを再帰的に削除する
  void delete_tree(BlockHuffmanNode node) {
    if (!node) return;
    if (!node->is_leaf) {
      for (int i = 0; i < q; i++) {
        delete_tree(node->children[i]);
      }
    }
    delete node;
  }

  // ノードと符号語を引数に取り、符号語を構築する
  // ノードが葉ならss_to_cwsに符号語を加える
  // ノードが葉でないなら、符号語に対応する番号を加えて、子ノードへ進む
  void build_code(BlockHuffmanNode node, vector<int> cw) {
    if (node->is_leaf) {
      ss_to_cws[node->symbols] = cw;
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
  // 元の数, 記号列と発生確率を受け取るコンストラクタ
  BlockHuffmanTree(int _q, const SymbolsPr& _symbolsPr) {
    q = _q;
    symbolsPr = _symbolsPr;
  }

  // ブロックハフマン符号語 : 記号列に対応する符号語を取り出せるmapを返す
  SymbolsToCodeWord huffman_code() {
    make_tree();
    build_code(root, {});
    delete_tree(root);
    return ss_to_cws;
  }
};