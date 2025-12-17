import init, { GameState, Player } from './pkg/latrones.js';

let game = null;

async function run() {
    await init();
    game = new GameState();
    renderBoard();
    updateUI();
    setupEventListeners();
}

function setupEventListeners() {
    document.getElementById('set-btn').addEventListener('click', () => {
        if (!game) {
            return;
        }
        game.set_starting_pieces();
        renderBoard();
        updateUI();
    });
    
    document.getElementById('reset-btn').addEventListener('click', () => {
        if (!game) {
            return;
        }
        game.reset();
        renderBoard();
        updateUI();
    });
}

function renderBoard() {
    const boardContainer = document.getElementById('board-container');
    boardContainer.innerHTML = '';
    
    const boardArray = game.get_board();
    const validMoves = game.get_valid_moves();
    const phase = game.phase;
    const selectedSquare = game.selected_square;
    
    // Create board with all labels inside
    const board = document.createElement('div');
    board.id = 'game-board';
    board.className = 'board';
    
    // Create top file labels (A-H) inside board
    const topLabels = document.createElement('div');
    topLabels.className = 'board-labels top-labels';
    const emptyTopLeft = document.createElement('div');
    emptyTopLeft.className = 'label-corner';
    topLabels.appendChild(emptyTopLeft);
    for (let col = 0; col < 8; col++) {
        const label = document.createElement('div');
        label.className = 'board-label file-label';
        label.textContent = String.fromCharCode(65 + col); // A-H
        topLabels.appendChild(label);
    }
    const emptyTopRight = document.createElement('div');
    emptyTopRight.className = 'label-corner';
    topLabels.appendChild(emptyTopRight);
    board.appendChild(topLabels);
    
    // Create board rows with rank labels
    for (let row = 7; row >= 0; row--) { // Start from row 7 (top) to row 0 (bottom)
        const boardRow = document.createElement('div');
        boardRow.className = 'board-row';
        
        // Add rank label on the left
        const rankLabelLeft = document.createElement('div');
        rankLabelLeft.className = 'board-label rank-label rank-label-left';
        rankLabelLeft.textContent = (row + 1).toString(); // 1-8
        boardRow.appendChild(rankLabelLeft);
        
        // Create board squares
        const squaresContainer = document.createElement('div');
        squaresContainer.className = 'board-squares';
        for (let col = 0; col < 8; col++) {
            const squareIndex = row * 8 + col;
            const square = document.createElement('div');
            square.className = 'square';
            square.dataset.index = squareIndex;
            
            const squareType = boardArray[squareIndex];
            let content = '';
            
            // Determine square content
            switch (squareType) {
                case 1: // LightPiece
                    content = '○';
                    square.className += ' light-piece';
                    break;
                case 2: // DarkPiece
                    content = '●';
                    square.className += ' dark-piece';
                    break;
                default: // Empty
                    square.className += ' empty';
                    content = '';
            }
            
            // Highlight selected piece
            if (selectedSquare !== null && selectedSquare !== undefined && selectedSquare === squareIndex) {
                square.className += ' selected';
            }
            
            // Highlight valid moves
            if (validMoves.includes(squareIndex)) {
                square.className += ' valid-move';
            }
            
            // Highlight selectable piece (only when no piece is selected)
            if (phase === 'movement' && (selectedSquare === null || selectedSquare === undefined) && 
                validMoves.includes(squareIndex) && (squareType === 1 || squareType === 2)) {
                square.className += ' selectable';
            }
            
            square.textContent = content;
            
            square.addEventListener('click', () => handleSquareClick(squareIndex));
            squaresContainer.appendChild(square);
        }
        boardRow.appendChild(squaresContainer);
        
        // Add rank label on the right
        const rankLabelRight = document.createElement('div');
        rankLabelRight.className = 'board-label rank-label rank-label-right';
        rankLabelRight.textContent = (row + 1).toString(); // 1-8
        boardRow.appendChild(rankLabelRight);
        
        board.appendChild(boardRow);
    }
    
    // Create bottom file labels (A-H) inside board
    const bottomLabels = document.createElement('div');
    bottomLabels.className = 'board-labels bottom-labels';
    const emptyBottomLeft = document.createElement('div');
    emptyBottomLeft.className = 'label-corner';
    bottomLabels.appendChild(emptyBottomLeft);
    for (let col = 0; col < 8; col++) {
        const label = document.createElement('div');
        label.className = 'board-label file-label';
        label.textContent = String.fromCharCode(65 + col); // A-H
        bottomLabels.appendChild(label);
    }
    const emptyBottomRight = document.createElement('div');
    emptyBottomRight.className = 'label-corner';
    bottomLabels.appendChild(emptyBottomRight);
    board.appendChild(bottomLabels);
    
    boardContainer.appendChild(board);
}

function updateUI() {
    const currentPlayer = game.current_player;
    const playerName = currentPlayer === Player.Light ? 'Light' : 'Dark';
    const playerNameEl = document.getElementById('player-name');
    playerNameEl.textContent = playerName;
    playerNameEl.className = currentPlayer === Player.Light ? '' : 'dark';
    
    const phaseNameEl = document.getElementById('phase-name');
    
    if (game.game_over) {
        const winner = game.winner;
        if (winner) {
            const winnerName = winner === Player.Light ? 'Light' : 'Dark';
            phaseNameEl.textContent = `${winnerName} win`;
        } else {
            phaseNameEl.textContent = 'Game Over';
        }
    } else {
        const phase = game.phase;
        phaseNameEl.textContent = phase === 'placement' ? 'Placement' : 'Movement';
    }
}

function handleSquareClick(squareIndex) {
    if (!game || game.game_over) {
        return;
    }
    
    const success = game.select_square(squareIndex);
    if (success) {
        renderBoard();
        updateUI();
    }
}

run().catch(console.error);

