"use strict";

var _interopRequireDefault = require("@babel/runtime/helpers/interopRequireDefault");

var _interopRequireWildcard = require("@babel/runtime/helpers/interopRequireWildcard");

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports["default"] = Cell;

var _react = _interopRequireWildcard(require("react"));

var _propTypes = _interopRequireDefault(require("prop-types"));

var _styledComponents = require("styled-components");

var _context = require("./context");

// expected props: row, col, answer, crossword, cellSize

/**
 * An individual-letter answer cell within the crossword grid.
 *
 * A `Cell` lives inside the SVG for a [`Crossword`](#crossword), and renders at
 * a location determined by the `row`, `col`, and `cellSize` properties from
 * `cellData` and `renderContext`.
 */
function Cell(_ref) {
  var cellData = _ref.cellData,
      onClick = _ref.onClick,
      focus = _ref.focus,
      highlight = _ref.highlight;

  var _useContext = (0, _react.useContext)(_context.CrosswordSizeContext),
      cellSize = _useContext.cellSize,
      cellPadding = _useContext.cellPadding,
      cellInner = _useContext.cellInner,
      cellHalf = _useContext.cellHalf,
      fontSize = _useContext.fontSize;

  var _useContext2 = (0, _react.useContext)(_styledComponents.ThemeContext),
      cellBackground = _useContext2.cellBackground,
      cellBorder = _useContext2.cellBorder,
      textColor = _useContext2.textColor,
      numberColor = _useContext2.numberColor,
      focusBackground = _useContext2.focusBackground,
      highlightBackground = _useContext2.highlightBackground;

  var handleClick = (0, _react.useCallback)(function (event) {
    event.preventDefault();

    if (onClick) {
      onClick(cellData);
    }
  }, [cellData, onClick]);
  var row = cellData.row,
      col = cellData.col,
      guess = cellData.guess,
      number = cellData.number;
  var x = col * cellSize;
  var y = row * cellSize;
  return /*#__PURE__*/_react["default"].createElement("g", {
    onClick: handleClick,
    style: {
      cursor: 'default',
      fontSize: fontSize + "px"
    }
  }, /*#__PURE__*/_react["default"].createElement("rect", {
    x: x + cellPadding,
    y: y + cellPadding,
    width: cellInner,
    height: cellInner,
    fill: focus ? focusBackground : highlight ? highlightBackground : cellBackground,
    stroke: cellBorder,
    strokeWidth: cellSize / 50
  }), number && /*#__PURE__*/_react["default"].createElement("text", {
    x: x + cellPadding * 4,
    y: y + cellPadding * 4,
    textAnchor: "start",
    dominantBaseline: "hanging",
    style: {
      fontSize: '50%',
      fill: numberColor
    }
  }, number), /*#__PURE__*/_react["default"].createElement("text", {
    x: x + cellHalf,
    y: y + cellHalf + 1 // +1 for visual alignment?
    ,
    textAnchor: "middle",
    dominantBaseline: "middle",
    style: {
      fill: textColor
    }
  }, guess));
}

process.env.NODE_ENV !== "production" ? Cell.propTypes = {
  /** the data specific to this cell */
  cellData: _propTypes["default"].shape({
    row: _propTypes["default"].number.isRequired,
    col: _propTypes["default"].number.isRequired,
    guess: _propTypes["default"].string.isRequired,
    number: _propTypes["default"].string
  }).isRequired,

  /** whether this cell has focus */
  focus: _propTypes["default"].bool,

  /** whether this cell is highlighted */
  highlight: _propTypes["default"].bool,

  /** handler called when the cell is clicked */
  onClick: _propTypes["default"].func
} : void 0;
Cell.defaultProps = {
  focus: false,
  highlight: false,
  onClick: null
}; // export default Cell;