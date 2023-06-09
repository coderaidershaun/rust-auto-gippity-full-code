use ai_functions::ai_function;

#[ai_function]
pub fn convert_user_input_to_goal(_user_request: &str) {
    /// Input: Takes in a user request
    /// Function: Converts user request into a short summarized goal
    /// Output: Prints goal. All outputs start with "build a website that ..."
    /// Example 1:
    ///   user_request = "I need a website that lets users login and logout. It needs to look fancy and accept payments."
    ///   OUTPUT = "build a website that handles users logging in and logging out and accepts payments"
    /// Example 2:
    ///   user_request = "Create something that stores crypto price data in a database using supabase and retrieves prices on the frontend."
    ///   OUTPUT = "build a website that fetches and stores crypto price data within a supabase setup including a frontend UI to fetch the data."
    println!(OUTPUT)
}
