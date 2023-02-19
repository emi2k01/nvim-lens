import { readFile } from "fs/promises";
import path from "path";

export type Config = {
  plugins: Plugin[];
};

export type Plugin = {
  id: string;
  url: string;
  name: string;
  colorschemes: string[];
};

export type ColorschemeWithPreview = {
  name: string;
  plugin: Plugin;
  previewHtml: string;
};

const PLUGINS_OUT_DIR = process.env.PLUGINS_OUT_DIR!;
const CONFIG_PATH = process.env.CONFIG_PATH!;
let _config: Config | undefined;

export async function getConfig(): Promise<Config> {
  if (!_config) {
    _config = JSON.parse(await readFile(CONFIG_PATH, "utf8"));
  }
  return _config!;
}

export async function getPlugins(): Promise<Plugin[]> {
  const config = await getConfig();
  return config.plugins;
}

export async function getColorschemesWithPreview(): Promise<
  ColorschemeWithPreview[]
> {
  const plugins = await getPlugins();
  const colorschemesWithPreview: ColorschemeWithPreview[] = [];
  for (const plugin of plugins) {
    const pluginDir = path.join(PLUGINS_OUT_DIR, plugin.id);
    for (const colorscheme of plugin.colorschemes) {
      const previewHtml = await readFile(
        path.join(pluginDir, colorscheme, "Rust.html"),
        "utf8"
      );
      colorschemesWithPreview.push({
        name: colorscheme,
        plugin,
        previewHtml,
      });
    }
  }
  return colorschemesWithPreview;
}
