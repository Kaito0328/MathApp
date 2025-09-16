#include <math.h>

#include <vector>
using namespace std;

#include <iostream>

#include "code.hpp"

// 区間の下限と上限のpair
typedef pair<double, double> Range;

// symbolをキーとして、区間を返すmap
typedef unordered_map<Symbol, Range> SymbolToRange;

// symbolと区間を管理するコンストラクタ
typedef struct {
  Alphabet alphabet;
  SymbolToRange s_to_rs;
} SymbolRange;

// 算術符号の、符号化、復号化を担うクラス
// symbolの発生確率を引数にとって初期化する
class ArithmeticCode {
 private:
  SymbolRange symbolRange;

  // symbolの発生確率を引数に取り、各symbolに対応する区間を返す関数
  SymbolRange sPr_to_sR(SymbolPr symbolPr) {
    SymbolRange symbolRange;
    symbolRange.alphabet = symbolPr.alphabet;

    double current_pr = 0;
    for (Symbol symbol : symbolRange.alphabet) {
      Range range;
      range.first = current_pr;
      current_pr += symbolPr.s_to_prs.at(symbol);
      range.second = current_pr;

      symbolRange.s_to_rs[symbol] = range;
    }
    return symbolRange;
  }

  // 記号列を引数にとり、対応する区間を返す
  Range cal_range(const Symbols& symbols) {
    Range currentRange = {0, 1};

    for (Symbol symbol : symbols) {
      double range_len = currentRange.second - currentRange.first;
      double min = currentRange.first;
      currentRange.first = min + symbolRange.s_to_rs[symbol].first * range_len;

      currentRange.second =
          min + symbolRange.s_to_rs[symbol].second * range_len;
    }
    return currentRange;
  }

  // 小数pを引数にとり、どの区間に入っているかを計算する。
  // その区間に入っているsymbolを返す
  Symbol in_range(double p) {
    for (Symbol symbol : symbolRange.alphabet) {
      Range range = symbolRange.s_to_rs.at(symbol);
      if (range.first <= p && range.second > p) return symbol;
    }
    return symbolRange.alphabet[0];
  }

 public:
  // symbolの発生確率を受け取るコンストラクタ
  ArithmeticCode(SymbolPr symbolPr) { symbolRange = sPr_to_sR(symbolPr); }

  // 符号化 : 記号列を引数にとり、対応する符号語を返す
  CodeWords encode(const Symbols& symbols) {
    Range range = cal_range(symbols);

    cout << "range: [" << range.first << ", " << range.second << ")" << endl;
    double mid = (range.first + range.second) / 2;
    double range_len = range.second - range.first;

    int length = ceil(-log2(range_len)) + 1;

    CodeWords cws = CodeWords(length);

    Range bin_range = {0, 1};
    for (int i = 0; i < length; i++) {
      double border = (bin_range.first + bin_range.second) / 2;
      if (mid < border) {
        cws[i] = 0;
        bin_range.second = border;
        continue;
      }
      cws[i] = 1;
      bin_range.first = border;
    }
    return cws;
  }

  // 復号 : 記号列の長さ, 符号語を引数に取り、記号列を返す
  Symbols decode(int length, const CodeWords& cws) {
    double p = 0;
    double bin = 1;
    for (int b : cws) {
      bin /= 2;
      p += b * bin;
    }

    Symbols symbols;

    for (int i = 0; i < length; i++) {
      Symbol symbol = in_range(p);
      Range range = symbolRange.s_to_rs[symbol];
      symbols += symbol;

      p = (p - range.first) / (range.second - range.first);
    }

    return symbols;
  }
};