const path = require('path');

module.exports = {
    mode: 'development',
    entry: {
        main: './src/index.tsx'
    },
    devServer: {
        static: {
            directory: path.join(__dirname, 'pub'),
        },
        compress: true,
        port: 9000,
        historyApiFallback: true
    },
    resolve: {
        alias: {
            '@': path.join(__dirname, 'src/')
        },
        extensions: ['.ts', '.tsx', '.js']
    },
    output: {
        filename: '[name].bundle.js',
        path: path.resolve(__dirname, 'dist'),
        clean: true
    },
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                exclude: /node_modules/,
                use: [
                    {
                        loader: 'babel-loader',
                        options: { presets: ['solid'] }
                    },
                    'ts-loader'
                ]
            },
            {
                test: /\.scss$/,
                use: ['style-loader', 'css-loader', 'sass-loader']
            }
        ]
    }
};
