const { createProxyMiddleware } = require('http-proxy-middleware');
module.exports = function(app) {
  app.use(
    '/api',
    createProxyMiddleware({
      target: process.env.REACT_APP_API_URL,
      changeOrigin: true,
    })
  );
  app.use(
    '/ipfs', 
    createProxyMiddleware({
      target: process.env.REACT_APP_IPFS_URL,
      changeOrigin: true,
    })
  );
  app.use(
    '/sandboxapi', 
    createProxyMiddleware({
      target: 'https://sandboxapi.rapyd.net/v1',
      changeOrigin: true,
    })
  );
};
