use std::{
    io::Write,
    path::Path,
    process::{Child, Command, Stdio},
};

use eyre::WrapErr;

use crate::{config::Plugin, COLUMNS, LINES, SAMPLES_EXTRACTION_PATH};

static CONFIG_DIR: &str = "/tmp/nvim/config";
static DATA_DIR: &str = "/tmp/nvim/data";
static STATE_DIR: &str = "/tmp/nvim/state";
static DEFAULT_PLUGINS: &str = r#"
"kyazdani42/nvim-web-devicons",
{
    "nvim-lualine/lualine.nvim",
    config = function()
        require("lualine").setup()
    end
},
"nvim-lua/plenary.nvim",
"nvim-telescope/telescope.nvim",
{
    "nvim-treesitter/nvim-treesitter",
    config = function()
        require'nvim-treesitter.configs'.setup({
          ensure_installed = { "rust", "typescript", "tsx" },
          sync_install = true,
          highlight = {
            enable = true,
          },
        })
    end
},
"#;
static OPTIONS: &str = r#"

vim.o.conceallevel = 2
vim.o.shortmess = vim.o.shortmess .. "c"
vim.o.termguicolors = true
vim.o.hidden = true
vim.o.updatetime = 16
vim.o.inccommand = "split"
vim.o.listchars = "tab:» ,extends:›,precedes:‹,nbsp:·,trail:·"
vim.o.completeopt = "menuone,noselect"
vim.o.pumheight = 20
vim.o.cmdheight = 1
vim.o.hidden = true
vim.o.scrolloff = 5
vim.o.splitbelow = true
vim.o.splitright = true
vim.o.ignorecase = true
vim.o.smartcase = true
vim.o.gdefault = true
vim.o.expandtab = true
vim.o.shiftwidth = 4
vim.o.tabstop = 4
vim.o.formatoptions = "croqnljb"
vim.o.foldlevelstart=99
vim.o.mouse = ""

vim.bo.expandtab = vim.o.expandtab
vim.bo.shiftwidth = vim.o.shiftwidth
vim.bo.tabstop = vim.o.tabstop
vim.bo.formatoptions = vim.o.formatoptions

vim.wo.signcolumn = "yes"
vim.wo.foldmethod = "indent"
vim.wo.foldexpr = "nvim_treesitter#foldexpr()"
vim.wo.number = true
vim.wo.list = true
vim.wo.listchars = vim.o.listchars
vim.wo.wrap = false
vim.wo.relativenumber = true
vim.wo.colorcolumn = "80"
"#;

pub(crate) fn command() -> Command {
    let mut command = Command::new("nvim");
    command
        .args(&["--listen", "localhost:5009"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .current_dir(SAMPLES_EXTRACTION_PATH)
        .env("LINES", LINES.to_string())
        .env("COLUMNS", COLUMNS.to_string())
        .env("XDG_CONFIG_HOME", CONFIG_DIR)
        .env("XDG_DATA_HOME", DATA_DIR)
        .env("XDG_STATE_HOME", STATE_DIR);
    command
}

pub(crate) fn spawn() -> Child {
    command().spawn().expect("failed to spawn nvim")
}

pub(crate) fn send_command(cmd: &str) {
    trace!(cmd, "sent cmd");
    Command::new("nvim")
        .args(&["--server", "localhost:5009", "--remote-send", cmd])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

pub(crate) fn setup(plugins: &[Plugin]) {
    let plugins_object = build_plugins_object(plugins);
    let lazy_bootstrap = format!(
        r#"
local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not vim.loop.fs_stat(lazypath) then
  vim.fn.system({{
    "git",
    "clone",
    "--filter=blob:none",
    "https://github.com/folke/lazy.nvim.git",
    "--branch=stable", -- latest stable release
    lazypath,
  }})
end
vim.opt.rtp:prepend(lazypath)
require("lazy").setup({plugins_object})
{OPTIONS}
"#
    );
    // setup init.lua
    std::fs::create_dir_all(CONFIG_DIR).unwrap();
    let init_lua = Path::new(CONFIG_DIR).join("nvim").join("init.lua");
    std::fs::create_dir(init_lua.parent().unwrap())
        .wrap_err("failed to create `nvim` directory")
        .unwrap();
    let mut init_lua = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&init_lua)
        .wrap_err_with(|| format!("failed to open {init_lua:?}"))
        .unwrap();
    init_lua.write_all(lazy_bootstrap.as_bytes()).unwrap();

    // install plugins
    Command::new("nvim")
        .args(&["--headless", "+Lazy! sync", "+qa"])
        .env("XDG_CONFIG_HOME", CONFIG_DIR)
        .env("XDG_DATA_HOME", DATA_DIR)
        .env("XDG_STATE_HOME", STATE_DIR)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn build_plugins_object(plugins: &[Plugin]) -> String {
    let mut plugins_buf = String::new();
    plugins_buf.push_str(DEFAULT_PLUGINS);
    for plugin in plugins {
        plugins_buf.push_str(&format!("\"{}\"", plugin.url));
        plugins_buf.push_str(",\n");
    }
    return format!("{{\n{plugins_buf}\n}}");
}
