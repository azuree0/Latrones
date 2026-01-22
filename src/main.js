import init, { GameState } from '../pkg/latrones.js'

let gameState = null
let wasmInitialized = false

// Initialize WASM
async function initWasm() {
  if (!wasmInitialized) {
    try {
      await init()
      wasmInitialized = true
      gameState = new GameState()
      render()
    } catch (error) {
      console.error('Error initializing WASM:', error)
    }
  }
}

// Game state management - thin wrappers around Rust methods
function selectSquare(squareIndex) {
  if (!gameState || gameState.game_over) return false
  const success = gameState.select_square(squareIndex)
  if (success) render()
  return success
}

function resetGame() {
  if (!gameState) return
  gameState.reset()
  render()
}

function setStartingPieces() {
  if (!gameState) return
  gameState.set_starting_pieces()
  render()
}

// Rendering - delegates to Rust
function render() {
  if (!gameState) {
    document.getElementById('root').innerHTML = '<div>Loading...</div>'
    return
  }
  
  const gameInfoEl = document.getElementById('game-info')
  if (gameInfoEl) {
    gameInfoEl.innerHTML = gameState.render_game_info_html()
  }
  
  const boardEl = document.getElementById('game-board')
  if (boardEl) {
    boardEl.innerHTML = gameState.render_game_board_html()
  }
}

// Make functions available globally for onclick handlers
window.selectSquare = selectSquare
window.resetGame = resetGame
window.setStartingPieces = setStartingPieces

// Initialize app
function initApp() {
  const root = document.getElementById('root')
  root.innerHTML = `
    <div class="container">
      <header>
        <h1>Latrones</h1>
      </header>
      <div id="game-info"></div>
      <div id="game-board"></div>
    </div>
  `
  initWasm()
}

// Start app when DOM is ready
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', initApp)
} else {
  initApp()
}
