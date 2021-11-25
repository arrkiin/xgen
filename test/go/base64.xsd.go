// Code generated by xgen. DO NOT EDIT.

package schema

import (
	"encoding/xml"
)

// MyType1 ...
type MyType1 []byte

// MyType2 ...
type MyType2 struct {
	XMLName    xml.Name `xml:"myType2"`
	LengthAttr int      `xml:"length,attr,omitempty"`
	Value      []byte   `xml:",chardata"`
}

// MyType3 ...
type MyType3 struct {
	XMLName    xml.Name `xml:"myType3"`
	LengthAttr int      `xml:"length,attr,omitempty"`
	Value      string   `xml:",chardata"`
}

// MyType4 ...
type MyType4 struct {
	XMLName   xml.Name `xml:"myType4"`
	Title     string   `xml:"title"`
	Blob      []byte   `xml:"blob"`
	Timestamp string   `xml:"timestamp"`
}

// MyType5 ...
type MyType5 string

// MyType6 ...
type MyType6 struct {
	CodeAttr       string `xml:"code,attr,omitempty"`
	IdentifierAttr int    `xml:"identifier,attr,omitempty"`
}

// MyType7 ...
type MyType7 struct {
	OriginAttr string `xml:"origin,attr"`
	Value      string `xml:",chardata"`
}

// TopLevel ...
type TopLevel struct {
	CostAttr        float64  `xml:"cost,attr,omitempty"`
	LastUpdatedAttr string   `xml:"LastUpdated,attr,omitempty"`
	Nested          *MyType7 `xml:"nested"`
	*MyType6
}
