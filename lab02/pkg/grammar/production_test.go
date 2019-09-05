package grammar

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestProduction_HasLeftRecursion(t *testing.T) {
	testCases := []struct {
		Input    Production
		Expected bool
	}{
		{
			Input: Production{
				LeftPart: []string{"A"},
				RightParts: [][]string{
					{"A", "b"},
					{"A", "c"},
					{"B"},
					{"C"},
				},
			},
			Expected: true,
		},
	}

	for _, testCase := range testCases {
		r := testCase.Input.HasLeftRecursion()
		assert.Equal(t, testCase.Expected, r)
	}
}

func TestProduction_IsEpsRule(t *testing.T) {
	testCases := []struct {
		Input    Production
		Expected bool
	}{
		{
			Input: Production{
				LeftPart: []string{"A"},
				RightParts: [][]string{
					{Eps},
				},
			},
			Expected: true,
		},
		{
			Input: Production{
				LeftPart: []string{"A"},
				RightParts: [][]string{
					{Eps},
					{"A", "B"},
				},
			},
			Expected: true,
		},
		{
			Input: Production{
				LeftPart: []string{"A"},
				RightParts: [][]string{
					{"b"},
					{"c"},
				},
			},
			Expected: false,
		},
	}

	for _, testCase := range testCases {
		r := testCase.Input.IsEps()
		assert.Equal(t, testCase.Expected, r)
	}
}

func TestProduction_WithoutLeftRecursion(t *testing.T) {
	testCases := []struct {
		Input    Production
		Expected interface{}
	}{
		{
			Input: Production{
				LeftPart: []string{"A"},
				RightParts: [][]string{
					{"A", "b"},
					{"A", "c"},
					{"B"},
					{"C"},
				},
			},
			Expected: map[string]Production{
				"A": {
					LeftPart: []string{"A"},
					RightParts: [][]string{
						{"B", "A'"},
						{"B"},
						{"C", "A'"},
						{"C"},
					},
				},
				"A'": {
					LeftPart: []string{"A'"},
					RightParts: [][]string{
						{"b", "A'"},
						{"b"},
						{"c", "A'"},
						{"c"},
					},
				},
			},
		},
	}

	for _, testCase := range testCases {
		r := testCase.Input.WithoutLeftRecursion()
		assert.Equal(t, testCase.Expected, r)
	}
}
