#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, PostResponse, QueryMsg};
use crate::post::{Post, POSTS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:unpopular_post";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let empy_vector = Vec::<Post>::new();
    POSTS.save(deps.storage, &empy_vector)?;
    //Just a formal initialization. Probably not needed but is nice.
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender.clone()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::NewPost { msg } => try_post(deps, msg, info.sender),
        ExecuteMsg::LikePost { id } => try_like(deps, id),
    }
}

pub fn try_post(deps: DepsMut, msg: String, sender: Addr) -> Result<Response, ContractError> {
    POSTS.update(deps.storage, |mut post| -> Result<_, ContractError> {
        // Insert the new post into the map
        let post_counter = post.len();
        post.push(Post {
            id: post_counter as u64,
            owner_id: sender,
            msg: msg,
            likes: 0 as u64,
        });
        Ok(post)
    })?;

    Ok(Response::new().add_attribute("method", "try_post"))
}

pub fn try_like(deps: DepsMut, id: u64) -> Result<Response, ContractError> {
    POSTS.update(deps.storage, |mut post| -> Result<_, ContractError> {
        post[id as usize].likes += 1;
        Ok(post)
    })?;

    Ok(Response::new().add_attribute("method", "try_like"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPosts {} => to_binary(&query_posts(deps)?),
    }
}

fn query_posts(deps: Deps) -> StdResult<PostResponse> {
    let posts = POSTS.load(deps.storage)?;
    Ok(PostResponse { posts: posts })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn test_proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        let query_res = query(deps.as_ref(), mock_env(), QueryMsg::GetPosts {}).unwrap();
        let post_response: PostResponse = from_binary(&query_res).unwrap();
        let post_list_len = post_response.posts.len();
        assert_eq!(post_list_len, 0, "Post list should be empty");
    }

    #[test]
    fn test_post() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::NewPost {
            msg: "50Cent should change his name to 50Dollar due to inflation".to_string(),
        };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetPosts {}).unwrap();
        let posts_map: PostResponse = from_binary(&res).unwrap();
        let post = posts_map.posts[0].clone();
        let post_list_len = posts_map.posts.len();
        assert_eq!(post_list_len, 1, "Post list should contain one post");
        assert_eq!(
            info.sender, post.owner_id,
            "Sender should be the owner of this post"
        );
    }

    #[test]
    fn test_like() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::NewPost {
            msg: "Hello Terra".to_string(),
        };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();

        let like = ExecuteMsg::LikePost { id: 0 };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), like).unwrap();
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetPosts {}).unwrap();
        let posts_map: PostResponse = from_binary(&res).unwrap();
        let post = posts_map.posts[0].clone();
        let post_list_len = posts_map.posts.len();
        assert_eq!(post_list_len, 1, "Post list should contain one post");
        assert_eq!(
            info.sender, post.owner_id,
            "Sender should be the owner of this post"
        );
        assert_eq!(post.likes, 1, "Number of likes should be 1")
    }
}
