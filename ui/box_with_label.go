package ui

import (
	"strings"

	"github.com/charmbracelet/lipgloss"
)

type BoxWithLabel struct {
	BoxStyle   lipgloss.Style
	LabelStyle lipgloss.Style
}

func NewDefaultBoxWithLabel() BoxWithLabel {
	return BoxWithLabel{
		BoxStyle: lipgloss.NewStyle().
			Border(lipgloss.RoundedBorder()).
			PaddingLeft(1),
		LabelStyle: lipgloss.NewStyle().
			PaddingTop(0).
			PaddingBottom(0).
			PaddingLeft(1).
			PaddingRight(1),
	}
}

func (b BoxWithLabel) Render(label, content string, width int, height int) string {
	var (
		border          = b.BoxStyle.GetBorderStyle()
		topBorderStyler = lipgloss.NewStyle().Foreground(b.BoxStyle.GetBorderTopForeground()).Render
		topLeft         = topBorderStyler(border.TopLeft)
		topRight        = topBorderStyler(border.TopRight)

		renderedLabel = b.LabelStyle.Render(label)
	)

	borderWidth := b.BoxStyle.GetHorizontalBorderSize()
	cellsShort := max(0, width+borderWidth-lipgloss.Width(topLeft+topRight+renderedLabel))
	gap := strings.Repeat(border.Top, cellsShort)
	top := topLeft + renderedLabel + topBorderStyler(gap) + topRight

	bottom := b.BoxStyle.
		BorderTop(false).
		Width(width).
		Height(height).
		Render(content)

	return top + "\n" + bottom
}
