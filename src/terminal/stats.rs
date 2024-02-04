pub struct Data {
    pub data: Vec<f64>
}

impl Data {
    pub fn mean(&self) -> f64{
        return self.data.iter().sum::<f64>() as f64 / self.data.len() as f64;
    }

    pub fn range(&self) -> Option<f64> {
        // Check if the input vector is not empty
        if self.data.is_empty() {
            return None; // Cannot calculate range for an empty vector
        }
        // Find the maximum and minimum values
        let max_value = *self.data.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let min_value = *self.data.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    
        // Calculate and return the range
        Some(max_value - min_value)
    }

    pub fn variance_n_std(&self) -> (f64, f64) {
        let mean = self.mean(); // Convert mean to integer for simplicity
        let variance: f64 = self.data.iter().map(|&x| ((x - mean) as f64).powi(2)).sum::<f64>() / self.data.len() as f64;
        let standard_deviation = variance.sqrt();
        (variance, standard_deviation)
    }

    pub fn percentiles(&self) -> (f64, f64, f64) {
        let mut sorted_data = self.data.clone();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let percentile_25 = self.percentile(&sorted_data, 25);
        let percentile_50 = self.percentile(&sorted_data, 50);
        let percentile_75 = self.percentile(&sorted_data, 75);
        (percentile_25, percentile_50, percentile_75)
    }

    fn percentile(&self, data: &Vec<f64>, p: usize) -> f64 {
        let index = (p as f64 / 100.0 * (data.len() - 1) as f64).round() as usize;
        data[index] as f64
    }
    
    pub fn skewness(&self) -> f64 {
        let n = self.data.len() as f64;
        let (_, std) = self.variance_n_std();
        let skewness: f64 = self.data
            .iter()
            .map(|&x| ((x - self.mean()) as f64 / std).powi(3))
            .sum::<f64>()
            / (n * (n - 1.0) * (n - 2.0))
            * n.sqrt();
        skewness
    }
    
    pub fn kurtosis(&self) -> f64 {
        let n = self.data.len() as f64;
        let (_, std) = self.variance_n_std();
        let kurtosis: f64 = self.data
            .iter()
            .map(|&x| ((x - self.mean()) as f64 / std).powi(4))
            .sum::<f64>()
            / ((n - 1.0) * (n - 2.0) * (n - 3.0))
            * (n * (n + 1.0) / ((n - 1.0) * (n - 2.0)).powi(2));
        kurtosis - 3.0
    }
}
