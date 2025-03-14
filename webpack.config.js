import path from 'path';

export default {
    mode: 'development',
    entry: './src/login.ts',
    output: {
        filename: 'login.js',
        path: path.resolve('./public/js'),
    },
    resolve: {
        extensions: ['.ts', '.js'],
    },
    module: {
        rules: [
            {
                test: /\.ts$/,
                use: 'ts-loader',
                exclude: /node_modules/,
            },
        ],
    },
    devServer: {
        static: './public/js',
        port: 3000,
    },
};
