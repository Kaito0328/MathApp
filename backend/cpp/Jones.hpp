#include <math.h>

#include <vector>
using namespace std;

#include <iostream>

#include "code.hpp"

// 区間の下限と上限のpair
typedef pair<double, double> Range;
// 小数の区間を整数に変換した累積頻度の区間
typedef pair<int, int> FRange;
// 記号に対応した区間を取り出せるmap
typedef unordered_map<Symbol, FRange> SymbolToFRange;

// 記号と累積頻度の区間を管理する構造体
// 情報源アルファベットと記号ン対応した累積頻度の区間を取り出せるmapを持つ
typedef struct {
  Alphabet alphabet;
  SymbolToFRange s_to_rs;
} SymbolFRange;

// Jones符号 (符号化・復号)ができるクラス
// シンボルと発生確率, スケールファクタを引き数に取り初期化する
class JonesCode {
 private:
  SymbolFRange symbolRange;
  int u;
  int om;

  // シンボルと発生確率を引き数に取り、シンボルと累積頻度を返す
  SymbolFRange sPr_to_sR(SymbolPr symbolPr) {
    SymbolFRange symbolRange;
    symbolRange.alphabet = symbolPr.alphabet;

    double pr_sum = 0;
    for (Symbol symbol : symbolRange.alphabet) {
      FRange range;
      range.first = floor(0.5 + (double)u * pr_sum);
      pr_sum += symbolPr.s_to_prs.at(symbol);
      range.second = floor(0.5 + (double)u * pr_sum);

      symbolRange.s_to_rs[symbol] = range;
    }
    return symbolRange;
  }

  // 整数eを受け取り、どの累積頻度の区間にあるかを探す
  // 区間に対応する記号を返す
  Symbol in_frange(int e) {
    for (Symbol symbol : symbolRange.alphabet) {
      FRange range = symbolRange.s_to_rs.at(symbol);
      if (range.first <= e && range.second > e) return symbol;
    }
    return symbolRange.alphabet[0];
  }

 public:
  // 記号と発生確率, スケールファクタを受け取るコンストラクタ
  JonesCode(SymbolPr symbolPr, int _u) {
    u = _u;
    om = 0;
    while (pow(2, om) < u) {
      om++;
    }
    symbolRange = sPr_to_sR(symbolPr);
  }

  // 符号化 :  記号列を引き数として、符号化した符号語を返す
  CodeWords encode(const Symbols& symbols) {
    long long X = 0;
    int Y = pow(2, om);
    int L = 0;

    for (Symbol symbol : symbols) {
      FRange range = symbolRange.s_to_rs.at(symbol);
      int V = floor((double)Y * range.second / u + 0.5) -
              floor((double)Y * range.first / u + 0.5);

      int s = 0;
      while (pow(2, om) > V * pow(2, s)) {
        s++;
      }

      X = (X + floor((double)Y * range.first / u + 0.5)) * pow(2, s);
      Y = V * pow(2, s);
      L = L + s;
    }

    long long code_number = X * pow(2, -om);
    CodeWords cws = CodeWords(L);

    long long bit_digit = pow(2, L);
    for (int i = 0; i < L; i++) {
      bit_digit /= 2;
      cws[i] = code_number / bit_digit;
      code_number %= bit_digit;
    }
    return cws;
  }

  // 復号 : 符号語を引き数として、復号した記号列を返す
  Symbols decode(CodeWords cws) {
    Symbols symbols = "";
    for (int i = 0; i < om; i++) {
      cws.push_back(1);
    }

    long long X = 0;
    long long bit_digit = 1;
    for (int i = om - 1; i >= 0; i--) {
      X += cws[i] * bit_digit;
      bit_digit *= 2;
    }

    int Y = pow(2, om);
    int L = 0;
    int s = 0;
    int index = om;
    const int size = (int)cws.size();

    while (index < size) {
      int e = floor((double)(u * (2 * X + 1) - 1) / (2 * Y));
      Symbol symbol = in_frange(e);
      symbols += symbol;

      FRange frange = symbolRange.s_to_rs[symbol];
      int Z = floor((double)Y * frange.first / u + 0.5);

      int V = floor((double)Y * frange.second / u + 0.5) - Z;

      s = 0;
      while (pow(2, om) > V * pow(2, s)) {
        s++;
      }
      X = X - Z;
      X = X * pow(2, s);
      Y = V * pow(2, s);

      while (s > 0 && index < size) {
        X += pow(2, s - 1) * cws[index];
        index++;
        s--;
      }
    }

    return symbols;
  }
};