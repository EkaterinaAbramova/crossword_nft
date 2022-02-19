"use strict";

var _interopRequireDefault = require("@babel/runtime/helpers/interopRequireDefault");

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.isAcross = isAcross;
exports.otherDirection = otherDirection;
exports.calculateExtents = calculateExtents;
exports.createEmptyGrid = createEmptyGrid;
exports.setCluesFilled = setCluesFilled;
exports.fillClues = fillClues;
exports.createGridData = createGridData;
exports.byNumber = byNumber;
exports.clearGuesses = clearGuesses;
exports.saveGuesses = saveGuesses;
exports.serializeGuesses = serializeGuesses;
exports.loadGuesses = loadGuesses;
exports.deserializeGuesses = deserializeGuesses;
exports.findCorrectAnswers = findCorrectAnswers;
exports.bothDirections = void 0;

var _extends2 = _interopRequireDefault(require("@babel/runtime/helpers/extends"));

var directionInfo = {
  across: {
    primary: 'col',
    orthogonal: 'row'
  },
  down: {
    primary: 'row',
    orthogonal: 'col'
  }
};
var bothDirections = Object.keys(directionInfo);
exports.bothDirections = bothDirections;

function isAcross(direction) {
  return direction === 'across';
}

function otherDirection(direction) {
  return isAcross(direction) ? 'down' : 'across';
}

function calculateExtents(data, direction) {
  var _ref2;

  var dir = directionInfo[direction];
  var primaryMax = 0;
  var orthogonalMax = 0;
  Object.entries(data[direction]).forEach(function (_ref) {
    var i = _ref[0],
        info = _ref[1];
    var primary = info[dir.primary] + info.answer.length - 1;

    if (primary > primaryMax) {
      primaryMax = primary;
    }

    var orthogonal = info[dir.orthogonal];

    if (orthogonal > orthogonalMax) {
      orthogonalMax = orthogonal;
    }
  });
  return _ref2 = {}, _ref2[dir.primary] = primaryMax, _ref2[dir.orthogonal] = orthogonalMax, _ref2;
}

var emptyCellData = {
  used: false,
  number: null,
  answer: '',
  guess: '',
  locked: false,
  // row: r,
  // col: c,
  across: null,
  down: null
};

function createEmptyGrid(size) {
  var gridData = Array(size); // Rather than [x][y] in column-major order, the cells are indexed as
  // [row][col] in row-major order.

  for (var r = 0; r < size; r++) {
    gridData[r] = Array(size);

    for (var c = 0; c < size; c++) {
      gridData[r][c] = (0, _extends2["default"])({}, emptyCellData, {
        row: r,
        col: c
      });
    }
  }

  return gridData;
}

function setCluesFilled(gridData, clues, data, direction) {
  var dir = directionInfo[direction];
  Object.entries(data[direction]).forEach(function (_ref3, idx) {
    var number = _ref3[0],
        info = _ref3[1];
    var rowStart = info.row,
        colStart = info.col,
        answer = info.answer;
    var isFilled = true;

    for (var i = 0; i < answer.length; i++) {
      var row = rowStart + (dir.primary === 'row' ? i : 0);
      var col = colStart + (dir.primary === 'col' ? i : 0);
      var cellData = gridData[row][col];

      if (cellData.guess === '') {
        isFilled = false;
      }
    }

    clues[direction][idx].isFilled = isFilled;

    if (isFilled) {
      clues[direction][idx].filledCounter += 1;
    }
  });
}

function fillClues(gridData, clues, data, direction) {
  var dir = directionInfo[direction];
  Object.entries(data[direction]).forEach(function (_ref4) {
    var number = _ref4[0],
        info = _ref4[1];
    var rowStart = info.row,
        colStart = info.col,
        clue = info.clue,
        answer = info.answer;

    for (var i = 0; i < answer.length; i++) {
      var row = rowStart + (dir.primary === 'row' ? i : 0);
      var col = colStart + (dir.primary === 'col' ? i : 0);
      var cellData = gridData[row][col]; // TODO?: check to ensure the answer is the same if it's already set?

      cellData.used = true;
      cellData.answer = answer[i];
      cellData[direction] = number;

      if (i === 0) {
        // TODO?: check to ensure the number is the same if it's already set?
        cellData.number = number;
      }
    }

    clues[direction].push({
      number: number,
      clue: clue
    });
  });
  clues[direction].sort(byNumber);
} // Given the "nice format" for a crossword, generate the usable data optimized
// for rendering and our interactivity.


function createGridData(data) {
  var acrossMax = calculateExtents(data, 'across');
  var downMax = calculateExtents(data, 'down');
  var size = Math.max.apply(Math, Object.values(acrossMax).concat(Object.values(downMax))) + 1;
  var gridData = createEmptyGrid(size); // Now fill with answers... and also collect the clues

  var clues = {
    across: [],
    down: []
  };
  fillClues(gridData, clues, data, 'across');
  fillClues(gridData, clues, data, 'down');
  return {
    size: size,
    gridData: gridData,
    clues: clues
  };
} // sort helper for clues...


function byNumber(a, b) {
  var aNum = Number.parseInt(a.number, 10);
  var bNum = Number.parseInt(b.number, 10);
  return aNum - bNum;
}

function clearGuesses(storageKey) {
  if (!window.localStorage) {
    return;
  }

  window.localStorage.removeItem(storageKey);
}

function saveGuesses(gridData, storageKey) {
  var _window = window,
      localStorage = _window.localStorage;

  if (!localStorage) {
    return;
  }

  var guesses = serializeGuesses(gridData);
  var saveData = {
    date: Date.now(),
    guesses: guesses
  };
  localStorage.setItem(storageKey, JSON.stringify(saveData));
}

function serializeGuesses(gridData) {
  var guesses = gridData.reduce(function (memo, row, r) {
    return row.reduce(function (memoInner, cellData, c) {
      var guess = cellData.guess;

      if (guess !== '') {
        memoInner[r + "_" + c] = cellData.guess;
      }

      return memoInner;
    }, memo);
  }, {});
  return guesses;
}

function loadGuesses(gridData, storageKey) {
  var _window2 = window,
      localStorage = _window2.localStorage;

  if (!localStorage) {
    return;
  }

  var saveRaw = localStorage.getItem(storageKey);

  if (!saveRaw) {
    return;
  }

  var saveData = JSON.parse(saveRaw); // TODO: check date for expiration?

  deserializeGuesses(gridData, saveData.guesses);
}

function deserializeGuesses(gridData, guesses) {
  Object.entries(guesses).forEach(function (_ref5) {
    var key = _ref5[0],
        val = _ref5[1];

    var _key$split = key.split('_'),
        r = _key$split[0],
        c = _key$split[1]; // ignore any out-of-bounds guesses!


    if (r <= gridData.length - 1 && c <= gridData[0].length - 1) {
      gridData[r][c].guess = val;
    }
  });
}

function findCorrectAnswers(data, gridData) {
  var correctAnswers = [];
  bothDirections.forEach(function (direction) {
    var across = isAcross(direction);
    Object.entries(data[direction]).forEach(function (_ref6) {
      var num = _ref6[0],
          info = _ref6[1];
      var row = info.row,
          col = info.col;
      var correct = true;

      for (var i = 0; i < info.answer.length; i++) {
        var r = across ? row : row + i;
        var c = across ? col + i : col;

        if (gridData[r][c].guess !== info.answer[i]) {
          correct = false;
          break;
        }
      }

      if (correct) {
        // same args as notifyCorrect: direction, number, answer
        correctAnswers.push([direction, num, info.answer]);
      }
    });
  });
  return correctAnswers;
}