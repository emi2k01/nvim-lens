import { readdir, readFile } from "fs/promises";
import { GetStaticPropsContext } from "next";
import path from "path";
import React from "react";

type Config = {
  plugins: {
    id: string;
    url: string;
    name: string;
    colorschemes: string[];
  }[];
};

type Screen = {
  name: string;
  html: string;
};

type ColorschemeShow = {
  name: string;
  screens: Screen[];
};

const PLUGINS_OUT_DIR = process.env.PLUGINS_OUT_DIR!;
const CONFIG_PATH = process.env.CONFIG_PATH!;
let _config: Config | undefined;

async function getConfig(): Promise<Config> {
  if (!_config) {
    _config = JSON.parse(await readFile(CONFIG_PATH, "utf8"));
  }
  return _config!;
}

export async function getStaticProps(context: GetStaticPropsContext) {
  const { pluginId } = context.params as { pluginId: string };
  const colorschemeOutputDir = path.join(PLUGINS_OUT_DIR, pluginId);
  const colorschemeDirNames = await readdir(colorschemeOutputDir);
  const colorschemeShows: ColorschemeShow[] = [];
  for (const colorschemeDirName of colorschemeDirNames) {
    const colorschemeDir = path.join(colorschemeOutputDir, colorschemeDirName);
    const screenFilenames = await readdir(colorschemeDir);
    const screens: Screen[] = [];
    for (const screenFilename of screenFilenames) {
      const screenFile = path.join(colorschemeDir, screenFilename);
      const screenHtml = await readFile(screenFile, "utf8");
      screens.push({
        name: screenFilename.replace(/\.html$/, ""),
        html: screenHtml,
      });
    }
    colorschemeShows.push({
      name: colorschemeDirName,
      screens,
    });
  }

  return {
    props: {
      colorschemeShows,
    },
  };
}

export async function getStaticPaths() {
  const config = await getConfig();
  return {
    paths: config.plugins.map((plugin) => ({
      params: {
        pluginId: plugin.id,
      },
    })),
    fallback: false,
  };
}

type PluginProps = {
  colorschemeShows: ColorschemeShow[];
};

export default function Plugin(props: PluginProps) {
  const [selectedShow, setSelectedShow] = React.useState<ColorschemeShow>(
    props.colorschemeShows[0]
  );

  return (
    <div className="flex items-start gap-x-4 py-12">
      <div className="flex flex-col gap-y-4">
        {selectedShow.screens.map((screen) => (
          <div key={screen.name} className="rounded-md overflow-hidden">
            <p className="text-xl">{screen.name}</p>
            <div
              className="rounded-md border border-white overflow-hidden mt-1"
              dangerouslySetInnerHTML={{ __html: screen.html }}
            ></div>
          </div>
        ))}
      </div>
      <ul className="flex flex-col gap-y-2 sticky top-12">
        {props.colorschemeShows.map((show) => (
          <li key={show.name}>
            <button
              className="px-4 py-2 bg-slate-600 rounded w-full"
              onClick={() => setSelectedShow(show)}
            >
              {show.name}
            </button>
          </li>
        ))}
      </ul>
    </div>
  );
}
