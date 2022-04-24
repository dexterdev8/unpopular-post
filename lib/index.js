module.exports = ({ wallets, refs, config, client }) => ({
  getPosts: () => client.query("unpopular_post", { get_posts: {} }),
  likePost: (signer=wallets.validator, post_id) => 
    client.execute(signer, "unpopular_post", { like_post: { id: post_id} }),
  post: (signer = wallets.validator, message) =>
    client.execute(signer, "unpopular_post", { new_post: {msg: message} }),
});
