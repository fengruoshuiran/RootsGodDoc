# 卡牌组

只有达成卡牌组的解锁条件，卡牌组才可能出现在升级备选中。解锁条件在卡牌组中可以查看模糊描述

```
card_group.is_unlock: bool
```

只有卡牌组中的卡牌达成它的解锁条件，才可能在 `洗入` 时获得。解锁条件可以在在组的卡牌上查看模糊描述

```
card_group.cards[card_name].is_unlock: bool
```

只有至少成功的 `占领`、`放置` 或 `洗入` 一次，才可以在图鉴中查看图像与描述。解锁但未“见过”时只能看到 ？卡背

```
card.is_appear: bool
```

只有卡牌的潜在分支结果真正实现过，才能在任何tooltip中看到相应分支的效果。否则直接隐藏不显示

```
card.branchs[branch_name]: bool
```

## 卡牌组联系

更好的了解与梳理卡牌组之间的潜在联系，请阅读 [tldraw页面](https://www.tldraw.com/r/8quNRPnCJYpRh9JxCG35c?d=v-506.150.2830.1418.page)

也可以在此处简单查看

<iframe src="https://www.tldraw.com/r/8quNRPnCJYpRh9JxCG35c?d=v-506.150.2830.1418.page" width="1080" height="720"></iframe>
