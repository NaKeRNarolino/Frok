import spaces from "color-space";
import {invoke} from "@tauri-apps/api/core";

export const cardBgColor = color(100, 97);

async function getBaseColor() {
    let color: number[] = await invoke("get_windows_accent_color");

    return color;
}

export async function color(saturation: number, lightness: number) {
    let baseColor = await getBaseColor();

    let baseColorHue = spaces.rgb.hsl(baseColor)[0];

    return `hsl(${baseColorHue}, ${saturation}%, ${lightness}%)`
}

export async function colorItself() {
    let baseColor = await getBaseColor();

    return `rgb(${baseColor[0]}, ${baseColor[1]}, ${baseColor[2]})`;
}