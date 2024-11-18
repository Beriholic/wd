package repo

import "github.com/Beriholic/wd/repo/model"

func MockQuery() (*model.Stardict, error) {
	return QueryWord("rust")
}

func QueryWord(wordName string) (*model.Stardict, error) {
	word := &model.Stardict{}

	err := DB.Model(&model.Stardict{}).Where("word = ?", wordName).First(&word).Error
	if err != nil {
		return nil, err
	}

	return word, nil
}
