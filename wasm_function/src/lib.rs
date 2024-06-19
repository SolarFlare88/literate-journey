use std::time::Instant;

use wasm_bindgen::prelude::*;

// 使用wasm_bindgen宏导出函数
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// rust 计算斐波那契数列.
/// 
/// # Arguments
///
/// * `n` - The position in the Fibonacci sequence to calculate.
///
/// # Examples
///
/// ```
/// let result = fibonacci(10);
/// assert_eq!(result, 55);
/// ```
#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n-2),
    }
}

#[wasm_bindgen]
pub struct CalculateResult {
    /** 值 */
    pub product: u64,
    /** 消耗时间 */
    pub duration_ms: u64,
}

#[wasm_bindgen]
#[cfg(not(target_arch = "wasm32"))]
pub fn calculate_product() -> Result<CalculateResult, JsValue> {
    let start = Instant::now();

    let mut product: u64 = 1;
    for i in 1..=1000 {
        product = product.checked_mul(i)
            .ok_or_else(|| JsValue::from_str("Multiplication overflow"))?;
    }

    let duration = start.elapsed();
    let duration_ms = duration.as_secs() * 1000 + u64::from(duration.subsec_millis());

    Ok(CalculateResult {
        product,
        duration_ms,
    })
}

// 单元测试模块
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_product() {
        // 测试正常情况
        let result = calculate_product().unwrap();
        assert_eq!(result.product, 0);  // 此处修改为你预期的乘积结果
        assert!(result.duration_ms > 0);  // 确保消耗时间大于0
    }

    #[test]
    fn test_overflow() {
        // 测试溢出情况
        let result = calculate_product();
        assert!(result.is_err());  // 确保返回错误
        assert_eq!(result.err().unwrap().as_string().unwrap(), "Multiplication overflow");
    }
}