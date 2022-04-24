import './App.css'

import { useEffect, useState } from 'react'
import {
  useWallet,
  useConnectedWallet,
  WalletStatus,
} from '@terra-money/wallet-provider'

import * as execute from './contract/execute'
import * as query from './contract/query'
import { ConnectWallet } from './components/ConnectWallet'

function App() {
  const [posts, setPosts] = useState(null)
  const [updating, setUpdating] = useState(true)
  const [postMessage, setPostMessage] = useState("")

  const { status } = useWallet()

  const connectedWallet = useConnectedWallet()

  useEffect(() => {
    const prefetch = async () => {
      if (connectedWallet) {
        //This should be a list of posts
        setPosts((await query.getPosts(connectedWallet)).posts)
      }
      setUpdating(false)
    }
    prefetch()
  }, [connectedWallet])

  const onClickPost = async () => {
    setUpdating(true)
    console.log(postMessage)
    await execute.post(connectedWallet, postMessage)
    setPosts((await query.getPosts(connectedWallet)).posts) //This should be a list of posts
    setUpdating(false)
  }

  return (
    <div className="App">
      <header className="App-header">
        {status === WalletStatus.WALLET_CONNECTED && (
          <div style={{ display: 'inline' }}>
            <input
              type="text"
              onChange={(e) => setPostMessage(+e.target.value)}
              value={postMessage}
            />
            <button onClick={onClickPost} type="button">
              {' '}
              post!{' '}
            </button>
          </div>
        )}
        <div style={{ display: 'inline' }}>
          {posts} {updating ? '(updating . . .)' : ''}
        </div>
        <ConnectWallet />
      </header>
    </div>
  )
}

export default App
