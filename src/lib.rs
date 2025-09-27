use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Input {
    projectName: String,
    industry: String,
    targetMarket: String,
    budgetUSD: f64,
    goal: String,
}

#[derive(Serialize)]
struct Output {
    marketAnalysis: String,
    financialPlan: String,
    strategy: String,
    conclusion: String,
}

#[wasm_bindgen]
pub fn run(input_json: &str) -> String {
    let input: Input = serde_json::from_str(input_json).unwrap_or(Input {
        projectName: "Project".into(),
        industry: "General".into(),
        targetMarket: "Global".into(),
        budgetUSD: 10000.0,
        goal: "Launch MVP".into(),
    });

    let market = format!(
        "Industry: {}. Target market: {}. Analyze TAM/SAM/SOM and competitors.",
        input.industry, input.targetMarket
    );

    let finance = format!(
        "Budget ${:.0}. Allocation: R&D 40%, Marketing 30%, Ops 20%, Reserve 10%.",
        input.budgetUSD
    );

    let strat = format!(
        "Go-to-market for '{}': positioning, pricing, channels, KPIs.",
        input.projectName
    );

    let concl = format!("Goal: {}. Next steps: validate, MVP, pilot test.", input.goal);

    let out = Output {
        marketAnalysis: market,
        financialPlan: finance,
        strategy: strat,
        conclusion: concl,
    };

    serde_json::to_string(&out).unwrap_or_else(|_| r#"{"error":"serialize failed"}"#.to_string())
}
