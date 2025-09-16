#include <math.h>

#include <iostream>
#include <unordered_map>
#include <vector>
using namespace std;

#include "code.hpp"

// lz78符号のノードを表す構造体
typedef struct Lz78Node_t {
  unordered_map<Symbol, Lz78Node_t*> children = {};
  int index = 0;
  Lz78Node_t(int _index) { index = _index; }
}* Lz78Node;

// 中間符号語 : 参照番号と記号のpair
typedef pair<int, Symbol> InternalCodeWord;
// 中間符号語列
typedef vector<InternalCodeWord> InternalCodeWords;

// lz78符号 (符号化, 復号) ができるクラス
// 中間符号語にエンコード, 中間符号語からデコードする関数も用意している
class Lz78_Code {
 private:
  const int ASCIIBIT = 8;

  // 中間符号語, 節点番号を引き数として、符号語を返す
  CodeWord internal_to_final(InternalCodeWord internal, int j) {
    int index = internal.first;
    Symbol symbol = internal.second;
    int code_number = symbol;
    code_number += pow(2, ASCIIBIT) * index;

    int bit_length = ceil(log2(j)) + ASCIIBIT;

    CodeWord cw = CodeWord(bit_length);
    long long bit_digit = pow(2, bit_length);
    for (int i = 0; i < bit_length; i++) {
      bit_digit /= 2;
      cw[i] = code_number / bit_digit;
      code_number = code_number % bit_digit;
    }
    return cw;
  }

  // 符号語を引き数として、中間符号語を返す
  InternalCodeWord final_to_internal(CodeWord cw) {
    InternalCodeWord internal;

    int index = 0;
    int index_length = (int)cw.size() - ASCIIBIT;
    long long bit_digit = 1;
    for (int i = index_length - 1; i >= 0; i--) {
      index += cw[i] * bit_digit;
      bit_digit *= 2;
    }

    int symbol_number = 0;
    bit_digit = 1;
    for (int i = (int)cw.size() - 1; i > index_length; i--) {
      symbol_number += cw[i] * bit_digit;
      bit_digit *= 2;
    }

    Symbol symbol = symbol_number;

    internal.first = index;
    internal.second = symbol;
    return internal;
  }

  // 親ノード, 節点番号, 記号を引き数として、ノードを作成し、そのノードを返す
  Lz78Node make_node(Lz78Node parent, int index, Symbol symbol) {
    Lz78Node new_node = new Lz78Node_t(index);
    parent->children[symbol] = new_node;
    return new_node;
  }

  // 再帰的にノードを削除する
  void delete_dict(Lz78Node node) {
    for (pair<Symbol, Lz78Node> child : node->children) {
      delete_dict(child.second);
    }
    delete node;
  }

 public:
  // 記号列を引き数として、符号化した中間符号語を返す
  InternalCodeWords internal_encode(const Symbols& symbols) {
    InternalCodeWords internal_cws;
    Lz78Node root = new Lz78Node_t(0);

    Lz78Node current_node = root;
    int current_index = 0;
    for (Symbol symbol : symbols) {
      auto child = current_node->children.find(symbol);
      if (child != current_node->children.end()) {
        current_node = child->second;
        continue;
      }

      current_index++;
      make_node(current_node, current_index, symbol);
      InternalCodeWord internal_cw = {current_node->index, symbol};
      internal_cws.push_back(internal_cw);

      current_node = root;
    }

    delete_dict(root);
    return internal_cws;
  }

  // 符号化 : 記号列を引き数として、符号化した符号語を返す
  CodeWords encode(const Symbols& symbols) {
    InternalCodeWords internal_cws = internal_encode(symbols);
    CodeWords cws = {};

    int j = 0;
    for (InternalCodeWord internal : internal_cws) {
      j++;
      CodeWord cw = internal_to_final(internal, j);
      cws.insert(cws.end(), cw.begin(), cw.end());
    }

    return cws;
  }

  // 中間符号語列を引き数として、復号した記号列を返す
  Symbols internal_decode(const InternalCodeWords& internal_cws) {
    Symbols symbols = "";
    Lz78Node root = new Lz78Node_t(0);

    vector<pair<Lz78Node, Symbols>> node_symbols = {};
    node_symbols.push_back({root, ""});

    for (InternalCodeWord internal : internal_cws) {
      pair<Lz78Node, Symbols> parent = node_symbols[internal.first];

      Lz78Node child = make_node(parent.first, internal.first, internal.second);
      Symbols child_symbols = parent.second + internal.second;
      node_symbols.push_back({child, child_symbols});

      symbols += child_symbols;
    }

    delete_dict(root);
    return symbols;
  }

  // 復号 : 符号語列を引き数として、復号した記号列を返す
  Symbols decode(const CodeWords& cws) {
    InternalCodeWords internal_cws = {};
    CodeWord cw = {};
    int current_index = 1;
    int bit_length = ceil(log2(current_index)) + ASCIIBIT;
    for (int i = 0; i < (int)cws.size(); i++) {
      cw.push_back(cws[i]);
      if ((int)cw.size() < bit_length) {
        continue;
      }

      InternalCodeWord internal = final_to_internal(cw);
      internal_cws.push_back(internal);
      current_index++;
      bit_length = ceil(log2(current_index)) + ASCIIBIT;
      cw = {};
    }

    Symbols symbols = internal_decode(internal_cws);
    return symbols;
  }
};

// 中間符号語を引き数として、出力する
void show_internal_encoded(const InternalCodeWords internal_encoded) {
  cout << "encoded: ";
  for (InternalCodeWord internal : internal_encoded) {
    cout << "(" << internal.first << ", " << internal.second << ") ";
  }
  cout << endl;
}

// 中間符号語を標準入力から受け取り返す
InternalCodeWords input_internal_cws(void) {
  int length;
  cout << "codewords length> ";
  cin >> length;

  InternalCodeWords internal_cws = InternalCodeWords(length);
  for (int i = 0; i < length; i++) {
    InternalCodeWord internal;
    cout << "node: ";
    cin >> internal.first;
    cout << "symbol: ";
    cin >> internal.second;
    internal_cws[i] = internal;
  }

  return internal_cws;
}