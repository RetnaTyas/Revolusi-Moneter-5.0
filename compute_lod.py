import json
from typing import List, Dict


def life_support_density(protein_g_per_kg: float, fat_g_per_kg: float, micronutrient_index: float) -> float:
    """Compute a simple life support density score."""
    energy = protein_g_per_kg * 4 + fat_g_per_kg * 9
    return energy * micronutrient_index


def lod_per_day(
    protein_g_per_kg: float,
    fat_g_per_kg: float,
    micronutrient_index: float,
    yield_per_cycle_kg: float,
    cycle_time_days: float,
    bias_factor: float = 1.0,
) -> float:
    """Calculate life support density per day."""
    lsd = life_support_density(protein_g_per_kg, fat_g_per_kg, micronutrient_index)
    daily_yield = yield_per_cycle_kg / cycle_time_days
    return round(lsd * daily_yield / 4.0 * bias_factor, 2)


def main() -> None:
    with open("lod_data_base.json") as f:
        base_data: List[Dict] = json.load(f)

    result: List[Dict] = []
    for entry in base_data:
        params = entry["parameters"]
        lod = lod_per_day(
            params["protein_g_per_kg"],
            params["fat_g_per_kg"],
            params["micronutrient_index"],
            params["yield_per_cycle_kg"],
            params["cycle_time_days"],
            params.get("bias_factor", 1.0),
        )
        result.append(
            {
                "commodity_id": entry["commodity_id"],
                "lod_per_day_nft": lod,
                "lod_per_day_virtual": lod,
                "lod_per_day_product": lod,
                "token_product_subtype": entry.get("token_product_subtype"),
            }
        )

    with open("lod_data.json", "w") as f:
        json.dump(result, f, indent=2)

    print("LOD data generated successfully.")


if __name__ == "__main__":
    main()
