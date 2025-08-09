use super::*;

#[cfg(test)]
mod tests;

impl<T: Scalar> Vector<T> {
    /// 指定されたデータから新しいVectorを生成する
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }

    /// ベクトルの次元（要素数）を返す
    pub fn dim(&self) -> usize {
        self.data.len()
    }

    /// 転置して行ベクトル（1行のMatrix）を生成する
    pub fn transpose(&self) -> Matrix<T> {
        Matrix::new(1, self.dim(), self.data.clone()).unwrap()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }

    pub fn argmax(&self) -> Option<usize>
    where
        T: PartialOrd,
    {
        if self.data.is_empty() {
            return None;
        }

        // 0番目のインデックスと値を初期値とする
        let mut max_index = 0;
        let mut max_value = &self.data[0];

        // 1番目からループを開始 (skip(1))
        for (i, current_value) in self.data.iter().enumerate().skip(1) {
            // 値を直接比較する
            if current_value > max_value {
                max_value = current_value;
                max_index = i;
            }
        }
        Some(max_index)
    }

    /// ベクトルの最小値のインデックスを返す
    pub fn argmin(&self) -> Option<usize>
    where
        T: PartialOrd,
    {
        if self.data.is_empty() {
            return None; // 空のベクトルの場合はNoneを返す
        }
        let mut min_index = 0;
        let mut min_value = &self.data[0];
        for (i, value) in self.data.iter().enumerate().skip(1) {
            if value < min_value {
                min_index = i;
                min_value = value;
            }
        }
        Some(min_index)
    }

    pub fn max(&self) -> Option<T>
    where
        T: PartialOrd + Copy,
    {
        if self.data.is_empty() {
            return None; // 空のベクトルの場合はNoneを返す
        }
        let mut max_value = self.data[0];
        for &value in self.data.iter().skip(1) {
            if value > max_value {
                max_value = value;
            }
        }
        Some(max_value)
    }

    pub fn min(&self) -> Option<T>
    where
        T: PartialOrd + Copy,
    {
        if self.data.is_empty() {
            return None; // 空のベクトルの場合はNoneを返す
        }
        let mut min_value = self.data[0];
        for &value in self.data.iter().skip(1) {
            if value < min_value {
                min_value = value;
            }
        }
        Some(min_value)
    }

    pub fn map<F, U>(&self, f: F) -> Vector<U>
    where
        F: Fn(&T) -> U, // クロージャは要素の参照を受け取り、新しい型の値を返す
        U: Scalar,      // 新しい要素の型もScalarである必要がある
    {
        let new_data = self.data.iter().map(f).collect();
        Vector::new(new_data)
    }
}
