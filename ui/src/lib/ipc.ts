// Typed wrapper around Tauri's invoke(). Centralizes the contract with the
// Rust side and falls back to mock data when running outside Tauri (e.g.
// during `astro dev`).

type InvokeFn = <T>(cmd: string, args?: Record<string, unknown>) => Promise<T>;

let cachedInvoke: InvokeFn | null = null;

async function getInvoke(): Promise<InvokeFn | null> {
  if (cachedInvoke) return cachedInvoke;
  // @ts-expect-error window.__TAURI_INTERNALS__ is set by Tauri when the
  // frontend runs inside the desktop shell. Outside (browser dev), it isn't.
  if (typeof window === "undefined" || !window.__TAURI_INTERNALS__) {
    return null;
  }
  const mod = await import("@tauri-apps/api/core");
  cachedInvoke = mod.invoke as InvokeFn;
  return cachedInvoke;
}

export interface Bottle {
  id: string;
  name: string;
  recipe?: string | null;
  prefix_subdir: string;
}

export interface Recipe {
  id: string;
  name: string;
  steam_app_id?: number | null;
  windows_version?: string | null;
}

export interface WineState {
  installed: boolean;
  version?: string | null;
  dll_count?: number | null;
  install_dir: string;
}

export async function listBottles(): Promise<Bottle[]> {
  const inv = await getInvoke();
  if (!inv) return MOCK_BOTTLES;
  return inv<Bottle[]>("list_bottles");
}

export async function createBottle(name: string, recipe?: string): Promise<Bottle> {
  const inv = await getInvoke();
  if (!inv) throw new Error("not running inside Tauri");
  return inv<Bottle>("create_bottle", { name, recipe: recipe ?? null });
}

export async function deleteBottle(id: string): Promise<void> {
  const inv = await getInvoke();
  if (!inv) throw new Error("not running inside Tauri");
  await inv<void>("delete_bottle", { id });
}

export async function listRecipes(): Promise<Recipe[]> {
  const inv = await getInvoke();
  if (!inv) return MOCK_RECIPES;
  return inv<Recipe[]>("list_recipes");
}

export async function wineState(): Promise<WineState> {
  const inv = await getInvoke();
  if (!inv) return MOCK_WINE;
  return inv<WineState>("wine_state");
}

export async function launchBottle(id: string, recipeId?: string): Promise<void> {
  const inv = await getInvoke();
  if (!inv) throw new Error("not running inside Tauri");
  await inv<void>("launch_bottle", { id, recipeId: recipeId ?? null });
}

// ----- Browser fallbacks ----------------------------------------------------

const MOCK_BOTTLES: Bottle[] = [];

const MOCK_RECIPES: Recipe[] = [
  { id: "norland", name: "Norland", steam_app_id: 1857090, windows_version: "win10" },
];

const MOCK_WINE: WineState = {
  installed: true,
  version: "wine-11.0 (mock)",
  dll_count: 1017,
  install_dir: "~/.vinaria/wine",
};
