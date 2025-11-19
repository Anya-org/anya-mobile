const esbuild = require('esbuild');

esbuild.build({
  entryPoints: ['src/ui/app.ts'],
  bundle: true,
  outfile: 'public/app.js',
  format: 'esm',
  platform: 'browser',
  // No external modules, so they will be bundled
}).catch(() => process.exit(1));
