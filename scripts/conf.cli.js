// starfall-cli config
// https://github.com/cenfun/starfall-cli

// const fs = require('fs');
// const path = require('path');

module.exports = {

    build: {

        webpackConfig: (conf, Util) => {
            // console.log(conf.externals);
            // conf.devtool = false;
        },

        after: (item, Util) => {

            // const filename = `${item.buildName}.js`;
            // // copy file
            // const fromPath = path.resolve(item.buildPath, filename);
            // if (!fs.existsSync(fromPath)) {
            //     Util.logRed(`ERROR: Not found: ${fromPath}`);
            //     return 1;
            // }
            // const toPath = path.resolve(item.devPath, filename);
            // // console.log(fromJs, toJs);
            // fs.cpSync(fromPath, toPath);

            // Util.logGreen(`Copied: ${toPath}`);

            return 0;
        }
    }
};
