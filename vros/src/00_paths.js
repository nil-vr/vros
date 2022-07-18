"use strict";

((window) => {
  const core = window.Deno.core;

  function expandPath(path) {
    return core.opSync("op_expand_path", path);
  }

  window.vros = {
    ...window.vros ?? {},
    expandPath,
  };
})(this);
