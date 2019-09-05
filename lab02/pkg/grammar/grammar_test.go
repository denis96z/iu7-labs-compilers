package grammar

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestGrammar_findEpsNonTerminals(t *testing.T) {
	testCases := []struct {
		Input    Grammar
		Expected []string
	}{
		{
			Input: Grammar{
				Terminals: []string{"d"},
				NonTerminals: []string{
					"A", "B", "C",
				},
				Productions: map[string]Production{
					"S": {
						LeftPart: []string{"S"},
						RightParts: [][]string{
							{"A", "B", "C"},
							{"D", "S"},
						},
					},
					"A": {
						LeftPart: []string{"A"},
						RightParts: [][]string{
							{Eps},
						},
					},
					"B": {
						LeftPart: []string{"B"},
						RightParts: [][]string{
							{"A", "C"},
						},
					},
					"C": {
						LeftPart: []string{"C"},
						RightParts: [][]string{
							{Eps},
						},
					},
					"D": {
						LeftPart: []string{"D"},
						RightParts: [][]string{
							{"d"},
						},
					},
				},
			},
			Expected: []string{
				"A", "B", "C", "S",
			},
		},
	}

	for _, testCase := range testCases {
		r := testCase.Input.findEpsNonTerminals()
		sort.Strings(r)

		assert.Equal(t, testCase.Expected, r)
	}
}

func TestGrammar_findEpsProductions(t *testing.T) {
	testCases := []struct {
		Input    Grammar
		Expected []Production
	}{
		{
			Input: Grammar{
				Productions: map[string]Production{
					"A": {
						LeftPart: []string{"A"},
						RightParts: [][]string{
							{Eps},
						},
					},
					"B": {
						LeftPart: []string{"B"},
						RightParts: [][]string{
							{"C"},
						},
					},
				},
			},
			Expected: []Production{
				{
					LeftPart: []string{"A"},
					RightParts: [][]string{
						{Eps},
					},
				},
			},
		},
	}

	for _, testCase := range testCases {
		r := testCase.Input.findEpsProductions()
		assert.Equal(t, testCase.Expected, r)
	}
}
