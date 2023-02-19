import { Plugin } from "@/types";
import { cn } from "@/util/cn";
import { getConfig } from "@/util/config";
import { readdir, readFile } from "fs/promises";
import { GetStaticPropsContext } from "next";
import Link from "next/link";
import { useRouter } from "next/router";
import path from "path";
import React from "react";

type Screen = {
  name: string;
  html: string;
};

type ColorschemeShow = {
  name: string;
  screens: Screen[];
};

type PluginPageProps = {
  colorschemeShows: ColorschemeShow[];
  plugin: Plugin;
};

const PLUGINS_OUT_DIR = process.env.PLUGINS_OUT_DIR!;

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

  const config = await getConfig();
  return {
    props: {
      colorschemeShows,
      plugin: config.plugins.find((plugin) => plugin.id === pluginId)!,
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

export default function PluginPage(props: PluginPageProps) {
  const router = useRouter();
  const [selectedShow, setSelectedShow] = React.useState<ColorschemeShow>(
    () =>
      props.colorschemeShows.find(
        (cs) => cs.name === router.query.colorscheme
      ) ?? props.colorschemeShows[0]
  );

  return (
    <div className="flex items-start gap-x-4 py-6">
      <div className="flex flex-col gap-y-4">
        {selectedShow.screens.map((screen) => (
          <div key={screen.name} className="rounded-md overflow-hidden">
            <p className="text-xl">{screen.name}</p>
            <div
              className="rounded-md border border-white overflow-hidden mt-1"
              dangerouslySetInnerHTML={{ __html: screen.html }}
            />
          </div>
        ))}
      </div>
      <ul className="flex flex-col gap-y-2 sticky top-6">
        <Link href={props.plugin.fullUrl} className="underline font-medium">
          {props.plugin.url}
        </Link>
        {props.colorschemeShows.map((show) => (
          <li key={show.name}>
            <button
              className={cn(
                "px-4 py-2 bg-slate-700 rounded w-full",
                selectedShow === show && "bg-slate-100 text-slate-800"
              )}
              onClick={() => {
                router.query.colorscheme = show.name;
                router.replace(router, "", { shallow: true });
                setSelectedShow(show);
              }}
            >
              {show.name}
            </button>
          </li>
        ))}
      </ul>
    </div>
  );
}
