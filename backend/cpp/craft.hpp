#include <unordered_map>
#include <vector>

using namespace std;

#include "code.hpp"

// クラフト木のノードを表す構造体
typedef struct CraftNode_t {
  CraftNode_t *parent, *left, *right;
  bool is_leaf;
  int symbol_index;

  CraftNode_t(CraftNode_t *parent)
      : parent(parent),
        left(nullptr),
        right(nullptr),
        is_leaf(false),
        symbol_index(0) {};
} *CraftNode;

// クラフト木を作成し、符号語の集合を返すクラス
// 情報源アルファベットの要素数, 符号語長列を引き数にとって、初期化する
class CraftTree {
 private:
  CodeBook codeBook;
  CraftNode root = new CraftNode_t(nullptr);
  int alphabet_size;
  vector<int> code_lengths;

  // 親ノードを引き数に取り、子ノードを作成する関数
  void make_children(CraftNode parent) {
    parent->left = new CraftNode_t(parent);
    parent->right = new CraftNode_t(parent);
  }

  // 深さ, ノード, 記号番号を引き数に取り、現在のノードから深さdepth,
  // 葉でないノードを探しだす関数 探索成功した場合、true,
  // 失敗した場合falseを返す
  // 探索した場合、最終ノードを葉に変え、記号番号を割り当てる
  bool find_craft_node(int depth, CraftNode node, int symbol_index) {
    if (node->is_leaf) return false;
    if (depth == 0) {
      node->is_leaf = true;
      node->symbol_index = symbol_index;
      return true;
    }

    if (node->left == nullptr) make_children(node);
    if (find_craft_node(depth - 1, node->left, symbol_index) ||
        find_craft_node(depth - 1, node->right, symbol_index))
      return true;
    return false;
  }

  // ノードと符号語を引き数に取り、符号語を構築する
  // ノードが葉ならcodeBookの対応する箇所に符号語を入れる
  // 葉でないなら、leftの符号語には0を追加、rightの符号語には1を追加する
  void build_code(CraftNode node, CodeWord code) {
    if (node->is_leaf) {
      codeBook[node->symbol_index] = code;
      return;
    }

    if (node->left == nullptr) return;
    CodeWord code_left = code;
    code_left.push_back(0);
    build_code(node->left, code_left);

    CodeWord code_right = code;
    code_right.push_back(1);
    build_code(node->right, code_right);
  }

  // クラフト木を作る関数
  void make_tree() {
    for (int i = 0; i < alphabet_size; i++) {
      if (find_craft_node(code_lengths[i], root, i + 1)) continue;
      return;
    }
  }

  // 動的確保により作成したノードを再帰的に削除する
  void delete_tree(CraftNode node) {
    if (!node) return;
    delete_tree(node->left);
    delete_tree(node->right);
    delete node;
  }

 public:
  // アルファベットの要素数と、記号長列を受け取るコンストラクタ
  CraftTree(int _alphabet_size, vector<int> _code_lengths) {
    alphabet_size = _alphabet_size;
    code_lengths = _code_lengths;
    codeBook = CodeBook(alphabet_size);
  }

  // クラフト木を作成し、符号語の集合を返す関数
  CodeBook craft_code() {
    make_tree();
    build_code(root, {});
    delete_tree(root);
    return codeBook;
  }
};