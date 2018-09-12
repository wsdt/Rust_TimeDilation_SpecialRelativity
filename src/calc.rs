/** Special relativity, time-dilation formula:

Observer time = (proper time)/(sqrt(1-(velocity/speed_of_light)^2))
*/

pub mod time_dilation_module {
    /** SPEED_OF_LIGHT often used as 'c' in physics/mathematical formulas.
        Following speed is true for light when travelling inside a vacuum. */
    const SPEED_OF_LIGHT: f32 = 299792.458; //in km/s

    pub fn calc_time_dilation(proper_time: f64, velocity: f64) -> f64 {
        (proper_time) / ((1 - (velocity / SPEED_OF_LIGHT).powi(2)).sqrt())
    }
}
