package grammar

//go:generate easyjson

import (
	"github.com/mailru/easyjson"
	"github.com/pkg/errors"
)

const (
	Eps = "#"
)

//easyjson:json
type Grammar struct {
	Terminals    []string              `json:"terminals"`
	NonTerminals []string              `json:"non_terminals"`
	Productions  map[string]Production `json:"productions"`
	Start        string                `json:"start"`
}

func ParseGrammar(s string) (*Grammar, error) {
	g := &Grammar{}
	if err := easyjson.Unmarshal([]byte(s), g); err != nil {
		return nil, errors.Wrap(err, "failed to unmarshal grammar")
	}
	return g, nil
}

func (v *Grammar) findEpsNonTerminals() []string {
	pEps := v.findEpsProductions()
	ntEps := v.selectEpsNonTerminals(pEps)

	for {
		ntEpsNew := v.extendEpsNonTerminalsSet(ntEps)
		if len(ntEps) == len(ntEpsNew) {
			break
		}
		ntEps = ntEpsNew
	}

	return ntEps
}

func (v *Grammar) findEpsProductions() []Production {
	pEps := make([]Production, 0)

	for _, p := range v.Productions {
		if p.IsEps() {
			pEps = append(pEps, p)
		}
	}

	return pEps
}

func (v *Grammar) selectEpsNonTerminals(pEps []Production) []string {
	ntEps := make([]string, 0)

	for _, p := range pEps {
		for _, s := range p.LeftPart {
			if v.isNonTerminal(s) {
				ntEps = append(ntEps, s)
			}
		}
	}

	return ntEps
}

func (v *Grammar) extendEpsNonTerminalsSet(ntEps []string) []string {
	newSet := copyStrSlice(ntEps)

	ext := make([]string, 0)
	for _, p := range v.Productions {
		if len(p.LeftPart) != 1 {
			continue
		}
		for _, pr := range p.RightParts {
			if isProductionEps(pr, ntEps) {
				ext = append(ext, p.LeftPart[0])
			}
		}
	}

	for _, ntNew := range ext {
		matchFound := false
		for _, nt := range ntEps {
			if nt == ntNew {
				matchFound = true
				break
			}
		}
		if !matchFound {
			newSet = append(newSet, ntNew)
		}
	}

	return newSet
}

func (v *Grammar) isTerminal(s string) bool {
	for _, t := range v.Terminals {
		if s == t {
			return true
		}
	}
	return false
}

func (v *Grammar) isNonTerminal(s string) bool {
	for _, nt := range v.NonTerminals {
		if s == nt {
			return true
		}
	}
	return false
}

func isProductionEps(pRight []string, ntEps []string) bool {
	for _, s := range pRight {
		matchFound := false
		for _, nt := range ntEps {
			if s == nt {
				matchFound = true
				continue
			}
		}
		if !matchFound {
			return false
		}
	}
	return true
}

func copyStrSlice(src []string) []string {
	dst := make([]string, len(src))
	copy(dst, src)
	return dst
}
