#include <iostream>

#include "code.hpp"
using namespace std;

// 条件付確率
typedef vector<vector<double>> CPr;

// アルファベットの要素数を引き数として、条件付確率を受け取り返す
CPr input_cp(int alphabet_size) {
  CPr cp(alphabet_size, vector<double>(alphabet_size, 0));
  for (int i = 0; i < alphabet_size; i++) {
    for (int j = 0; j < alphabet_size; j++) {
      cout << "cp_" << i + 1 << j + 1 << "> ";
      cin >> cp[i][j];
    }
  }
  return cp;
}

// アルファベットの要素数を引き数として、記号に対応した要素番号を取り出せるmapを返す
unordered_map<Symbol, int> map_index(Alphabet alphabet) {
  unordered_map<Symbol, int> s_to_index;
  for (int i = 0; i < (int)alphabet.size(); i++) {
    s_to_index[alphabet[i]] = i;
  }
  return s_to_index;
}

// マルコフ情報源でのブロック化した記号と発生確率を返すクラス
class Markov {
 private:
  SymbolPr symbolPr = SymbolPr();
  int alphabet_size;
  unordered_map<Symbol, int> s_to_index;
  CPr cp;

  // ブロック長とブロック化された記号列を引き数として、発生確率を求めて返す
  Pr cal_markov_pr(int length, Symbols symbols) {
    Symbol first_symbol = symbols[0];
    Symbol pre_symbol = first_symbol;

    double pr = symbolPr.s_to_prs[first_symbol];

    for (int i = 1; i < length; i++) {
      Symbol current_symbol = symbols[i];
      pr *= cp[s_to_index[pre_symbol]][s_to_index[current_symbol]];
      pre_symbol = current_symbol;
    }

    return pr;
  }

 public:
  // 記号と発生確率を受け取るコンストラクタ
  // 条件付確率をここで標準入力から受け取る
  Markov(const SymbolPr& _symbolPr) {
    symbolPr = _symbolPr;
    alphabet_size = symbolPr.alphabet.size();
    s_to_index = map_index(symbolPr.alphabet);
    cp = input_cp(alphabet_size);
  }

  // ブロック長を引き数として、ブロック化した記号列と発生確率を返す
  SymbolsPr markov_symbols_pr(int length) {
    SymbolsPr symbolsPr;
    symbolsPr.block_alphabet = enum_symbols(length, symbolPr.alphabet);
    for (Symbols symbols : symbolsPr.block_alphabet) {
      symbolsPr.ss_to_prs[symbols] = cal_markov_pr(length, symbols);
    }
    return symbolsPr;
  }
};