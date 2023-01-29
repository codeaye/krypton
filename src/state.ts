import { writable } from "svelte/store";
import { Store } from "tauri-plugin-store-api";

const store = new Store(".previous.dat");

async function createSharedState<T>(value: T, key: string) {
    const retrieved = await store.get<null | T>(key) ?? value;
    const { subscribe, set } = writable(retrieved);
    let curr_val = retrieved;

    await store.set(key, retrieved)

    return {
        subscribe,
        reset: () => {
            set(value);
            curr_val = value
        },
        set: (new_value: T) => {
            set(new_value);
            curr_val = new_value
        },
        sync: async () => {
            await store.set(key, curr_val);
        }
    };
}


let input_text = await createSharedState(`{ "cool_app": true }`, "input_text");
let input_format = await createSharedState('Json', "input_format");
let output_format = await createSharedState('Toml', "output_format");
let output_text = writable("")

export async function sync() {
    await input_text.sync();
    await input_format.sync();
    await output_format.sync();
    await store.save()
}

export {
    store,
    input_text,
    input_format,
    output_text,
    output_format
}