package grammar

//go:generate easyjson

import (
	"strings"
)

//easyjson:json
type Production struct {
	LeftPart   []string   `json:"left_part"`
	RightParts [][]string `json:"right_parts"`
	alphabet   []string   `json:"-"`
}

func (v Production) HasLeftRecursion() bool {
	if len(v.LeftPart) != 1 {
		return false
	}

	left := v.LeftPart[0]
	for _, part := range v.RightParts {
		if left == part[0] {
			return true
		}
	}

	return false
}

func (v Production) IsEps() bool {
	if len(v.LeftPart) != 1 {
		return false
	}

	for _, part := range v.RightParts {
		if len(part) == 1 && part[0] == Eps {
			return true
		}
	}

	return false
}

func (v Production) WithoutLeftRecursion() map[string]Production {
	if !v.HasLeftRecursion() {
		return map[string]Production{
			strings.Join(v.LeftPart, " "): v,
		}
	}

	left := v.LeftPart[0]
	leftNew := left + "'"

	rec := make([][]string, 0)
	nrec := make([][]string, 0)

	for _, right := range v.RightParts {
		if left == right[0] {
			rec = append(rec, right)
		} else {
			nrec = append(nrec, right)
		}
	}

	nrecNew := make([][]string, 0)
	for _, p := range nrec {
		n := len(p)

		newP := make([]string, n+1)
		copy(newP, p)
		newP[n] = leftNew

		nrecNew = append(nrecNew, newP, p)
	}

	recNew := make([][]string, 0)
	for _, p := range rec {
		n := len(p)

		newP := make([]string, n)
		copy(newP, p[1:])
		newP[n-1] = leftNew

		recNew = append(recNew, newP, p[1:])
	}

	newProductions := map[string]Production{
		left: {
			LeftPart:   []string{left},
			RightParts: nrecNew,
		},
		leftNew: {
			LeftPart:   []string{leftNew},
			RightParts: recNew,
		},
	}

	return newProductions
}
