export type Config = {
  plugins: Plugin[];
};

export type Plugin = {
  id: string;
  url: string;
  fullUrl: string;
  name: string;
  colorschemes: string[];
};

export type ColorschemeWithPreview = {
  name: string;
  plugin: Plugin;
  previewHtml: string;
};
