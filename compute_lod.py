import json
from typing import Dict, List

def life_support_density(protein_g_per_kg: float, fat_g_per_kg: float, micronutrient_index: float) -> float:
    """Compute a simple life support density score."""
    energy = protein_g_per_kg * 4 + fat_g_per_kg * 9
    return energy * micronutrient_index

def lod_per_day(protein_g_per_kg: float, fat_g_per_kg: float, micronutrient_index: float,
                yield_per_cycle_kg: float, cycle_time_days: float, bias_factor: float = 1.0) -> float:
    lsd = life_support_density(protein_g_per_kg, fat_g_per_kg, micronutrient_index)
    daily_yield = yield_per_cycle_kg / cycle_time_days
    return round(lsd * daily_yield / 4.0 * bias_factor, 2)

commodities: List[Dict] = [
    {
        "commodity_id": "KAMBING",
        "parameters": {
            "protein_g_per_kg": 270,
            "fat_g_per_kg": 200,
            "micronutrient_index": 0.9,
            "yield_per_cycle_kg": 25,
            "cycle_time_days": 365,
            "market_price_usd_per_kg": 8.5,
            "bias_factor": 1.0
        }
    },
    {
        "commodity_id": "ITIK",
        "parameters": {
            "protein_g_per_kg": 240,
            "fat_g_per_kg": 160,
            "micronutrient_index": 0.85,
            "yield_per_cycle_kg": 2,
            "cycle_time_days": 50,
            "market_price_usd_per_kg": 3.5,
            "bias_factor": 0.85
        }
    },
    {
        "commodity_id": "AYAM",
        "parameters": {
            "protein_g_per_kg": 270,
            "fat_g_per_kg": 90,
            "micronutrient_index": 0.8,
            "yield_per_cycle_kg": 2,
            "cycle_time_days": 42,
            "market_price_usd_per_kg": 2.25,
            "bias_factor": 0.75
        }
    },
    {
        "commodity_id": "PADI",
        "parameters": {
            "protein_g_per_kg": 60,
            "fat_g_per_kg": 5,
            "micronutrient_index": 0.7,
            "yield_per_cycle_kg": 3,
            "cycle_time_days": 120,
            "market_price_usd_per_kg": 1.1,
            "bias_factor": 1.0
        }
    },
    {
        "commodity_id": "SAPI",
        "parameters": {
            "protein_g_per_kg": 250,
            "fat_g_per_kg": 180,
            "micronutrient_index": 0.95,
            "yield_per_cycle_kg": 300,
            "cycle_time_days": 730,
            "market_price_usd_per_kg": 7.0,
            "bias_factor": 1.1
        }
    },
    {
        "commodity_id": "TELUR_AYAM",
        "parameters": {
            "protein_g_per_kg": 120,
            "fat_g_per_kg": 100,
            "micronutrient_index": 0.9,
            "yield_per_cycle_kg": 300,
            "cycle_time_days": 365,
            "market_price_usd_per_kg": 1.5,
            "bias_factor": 0.9
        }
    },
    {
        "commodity_id": "SUSU_SAPI",
        "parameters": {
            "protein_g_per_kg": 33,
            "fat_g_per_kg": 36,
            "micronutrient_index": 0.8,
            "yield_per_cycle_kg": 8000,
            "cycle_time_days": 305,
            "market_price_usd_per_kg": 0.5,
            "bias_factor": 1.0
        }
    },
    {
        "commodity_id": "JAGUNG",
        "parameters": {
            "protein_g_per_kg": 90,
            "fat_g_per_kg": 45,
            "micronutrient_index": 0.6,
            "yield_per_cycle_kg": 5,
            "cycle_time_days": 100,
            "market_price_usd_per_kg": 0.9,
            "bias_factor": 1.0
        }
    },
    {
        "commodity_id": "KACANG_TANAH",
        "parameters": {
            "protein_g_per_kg": 250,
            "fat_g_per_kg": 490,
            "micronutrient_index": 0.85,
            "yield_per_cycle_kg": 2.5,
            "cycle_time_days": 110,
            "market_price_usd_per_kg": 1.8,
            "bias_factor": 1.0
        }
    },
    {
        "commodity_id": "MADU",
        "parameters": {
            "protein_g_per_kg": 5,
            "fat_g_per_kg": 0,
            "micronutrient_index": 1.0,
            "yield_per_cycle_kg": 50,
            "cycle_time_days": 180,
            "market_price_usd_per_kg": 10.0,
            "bias_factor": 1.2
        }
    }
]

for entry in commodities:
    p = entry["parameters"]
    lod = lod_per_day(
        p["protein_g_per_kg"],
        p["fat_g_per_kg"],
        p["micronutrient_index"],
        p["yield_per_cycle_kg"],
        p["cycle_time_days"],
        p.get("bias_factor", 1.0)
    )
    entry["lod_per_day_nft"] = lod
    entry["lod_per_day_virtual"] = lod
    entry["lod_per_day_product"] = lod

with open("lod_data.json", "w") as f:
    json.dump(commodities, f, indent=2)
