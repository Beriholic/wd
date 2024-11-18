package model

const TableName = "stardict"

type Stardict struct {
	ID          int32  `gorm:"column:id;primaryKey" json:"id"`
	Word        string `gorm:"column:word;not null" json:"word"`
	Sw          string `gorm:"column:sw;not null" json:"sw"`
	Phonetic    string `gorm:"column:phonetic" json:"phonetic"`
	Definition  string `gorm:"column:definition" json:"definition"`
	Translation string `gorm:"column:translation" json:"translation"`
	Pos         string `gorm:"column:pos" json:"pos"`
	Collins     int32  `gorm:"column:collins" json:"collins"`
	Oxford      int32  `gorm:"column:oxford" json:"oxford"`
	Tag         string `gorm:"column:tag" json:"tag"`
	Bnc         int32  `gorm:"column:bnc" json:"bnc"`
	Frq         int32  `gorm:"column:frq" json:"frq"`
	Exchange    string `gorm:"column:exchange" json:"exchange"`
	Detail      string `gorm:"column:detail" json:"detail"`
	Audio       string `gorm:"column:audio" json:"audio"`
}

func (*Stardict) TableName() string {
	return TableName
}
