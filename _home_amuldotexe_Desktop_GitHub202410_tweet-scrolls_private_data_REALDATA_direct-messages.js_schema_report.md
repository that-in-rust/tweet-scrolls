# JSON Schema Discovery Report

**Total items analyzed:** 1000

**Summary:**
- Total fields: 132
- Required fields: 13
- Optional fields: 119
- Mixed-type fields: 0

## Field Details

### `dmConversation`
- **Types:** {"Object"}
- **Optional:** false (1000/1000 items)
- **Samples:** ["{2 fields}", "{2 fields}", "{2 fields}"]

### `dmConversation.conversationId`
- **Types:** {"String"}
- **Optional:** false (1000/1000 items)
- **Samples:** ["3382-1132151165410455552", "3977-1132151165410455552", "1315501-1132151165410455552"]

### `dmConversation.messages`
- **Types:** {"Array"}
- **Optional:** false (1000/1000 items)
- **Samples:** ["[94 items]", "[2 items]", "[2 items]"]

### `dmConversation.messages[0].messageCreate`
- **Types:** {"Object"}
- **Optional:** false (1000/1000 items)
- **Samples:** ["{9 fields}", "{9 fields}", "{9 fields}"]

### `dmConversation.messages[0].messageCreate.createdAt`
- **Types:** {"String"}
- **Optional:** false (1000/1000 items)
- **Samples:** ["2025-05-27T15:22:27.518Z", "2025-01-15T16:59:25.478Z", "2022-01-31T13:25:06.103Z"]

### `dmConversation.messages[0].messageCreate.editHistory`
- **Types:** {"Array"}
- **Optional:** false (996/1000 items)
- **Samples:** ["[0 items]", "[0 items]", "[0 items]"]

### `dmConversation.messages[0].messageCreate.editHistory[0].createdAtSec`
- **Types:** {"String"}
- **Optional:** true (1/1000 items)
- **Samples:** ["1747329360"]

### `dmConversation.messages[0].messageCreate.editHistory[0].editedText`
- **Types:** {"String"}
- **Optional:** true (1/1000 items)
- **Samples:** ["Ah makes sense. I happened to have read the first "]

### `dmConversation.messages[0].messageCreate.id`
- **Types:** {"String"}
- **Optional:** false (1000/1000 items)
- **Samples:** ["1927384914816532581", "1879574119072899535", "1488141288121978886"]

### `dmConversation.messages[0].messageCreate.mediaUrls`
- **Types:** {"Array"}
- **Optional:** false (1000/1000 items)
- **Samples:** ["[0 items]", "[0 items]", "[0 items]"]

### `dmConversation.messages[0].messageCreate.reactions`
- **Types:** {"Array"}
- **Optional:** false (1000/1000 items)
- **Samples:** ["[1 items]", "[0 items]", "[0 items]"]

### `dmConversation.messages[0].messageCreate.reactions[0].createdAt`
- **Types:** {"String"}
- **Optional:** true (379/1000 items)
- **Samples:** ["2025-05-27T15:28:42.241Z", "2023-11-29T08:34:21.270Z", "2024-10-14T16:50:35.075Z"]

### `dmConversation.messages[0].messageCreate.reactions[0].eventId`
- **Types:** {"String"}
- **Optional:** true (379/1000 items)
- **Samples:** ["1927386486816542720", "1729780825577971712", "1845869822992629760"]

### `dmConversation.messages[0].messageCreate.reactions[0].reactionKey`
- **Types:** {"String"}
- **Optional:** true (379/1000 items)
- **Samples:** ["like", "like", "excited"]

### `dmConversation.messages[0].messageCreate.reactions[0].senderId`
- **Types:** {"String"}
- **Optional:** true (379/1000 items)
- **Samples:** ["3382", "1132151165410455552", "1132151165410455552"]

### `dmConversation.messages[0].messageCreate.recipientId`
- **Types:** {"String"}
- **Optional:** false (1000/1000 items)
- **Samples:** ["3382", "3977", "1315501"]

### `dmConversation.messages[0].messageCreate.senderId`
- **Types:** {"String"}
- **Optional:** false (1000/1000 items)
- **Samples:** ["1132151165410455552", "1132151165410455552", "1132151165410455552"]

### `dmConversation.messages[0].messageCreate.text`
- **Types:** {"String"}
- **Optional:** false (1000/1000 items)
- **Samples:** ["https://t.co/nsVswPpSDi\n\nI have been thinking abou", "Anumodna 🙏", "Thanks for sharing KP"]

### `dmConversation.messages[0].messageCreate.urls`
- **Types:** {"Array"}
- **Optional:** false (1000/1000 items)
- **Samples:** ["[1 items]", "[0 items]", "[0 items]"]

### `dmConversation.messages[0].messageCreate.urls[0].display`
- **Types:** {"String"}
- **Optional:** true (153/1000 items)
- **Samples:** ["x.com/hnshah/status/…", "eightify.app", "amuldotexe.fermion.app/course/jsg2024"]

### `dmConversation.messages[0].messageCreate.urls[0].expanded`
- **Types:** {"String"}
- **Optional:** true (153/1000 items)
- **Samples:** ["https://x.com/hnshah/status/1927383046421893182", "https://eightify.app", "https://amuldotexe.fermion.app/course/jsg2024"]

### `dmConversation.messages[0].messageCreate.urls[0].url`
- **Types:** {"String"}
- **Optional:** true (153/1000 items)
- **Samples:** ["https://t.co/nsVswPpSDi", "https://t.co/Xe1NgsSqta", "https://t.co/wHND8UD4Oj"]

### `dmConversation.messages[0].messageCreate.urls[1].display`
- **Types:** {"String"}
- **Optional:** true (18/1000 items)
- **Samples:** ["x.com/amuldotexe/sta…", "insightful.in", "twitter.com/brickywhat/sta…"]

### `dmConversation.messages[0].messageCreate.urls[1].expanded`
- **Types:** {"String"}
- **Optional:** true (18/1000 items)
- **Samples:** ["https://x.com/amuldotexe/status/183777811841914087", "http://insightful.in", "https://twitter.com/brickywhat/status/172722540730"]

### `dmConversation.messages[0].messageCreate.urls[1].url`
- **Types:** {"String"}
- **Optional:** true (18/1000 items)
- **Samples:** ["https://t.co/SWKhxjajQi", "https://t.co/ZeUZL6Khxn", "https://t.co/Fcyww2U1V1"]

### `dmConversation.messages[0].messageCreate.urls[2].display`
- **Types:** {"String"}
- **Optional:** true (1/1000 items)
- **Samples:** ["g.co/kgs/AyGmss"]

### `dmConversation.messages[0].messageCreate.urls[2].expanded`
- **Types:** {"String"}
- **Optional:** true (1/1000 items)
- **Samples:** ["https://g.co/kgs/AyGmss"]

### `dmConversation.messages[0].messageCreate.urls[2].url`
- **Types:** {"String"}
- **Optional:** true (1/1000 items)
- **Samples:** ["https://t.co/Yty7bN9nwa"]

### `dmConversation.messages[0].messageCreate.urls[3].display`
- **Types:** {"String"}
- **Optional:** true (1/1000 items)
- **Samples:** ["insightful.in"]

### `dmConversation.messages[0].messageCreate.urls[3].expanded`
- **Types:** {"String"}
- **Optional:** true (1/1000 items)
- **Samples:** ["http://insightful.in"]

### `dmConversation.messages[0].messageCreate.urls[3].url`
- **Types:** {"String"}
- **Optional:** true (1/1000 items)
- **Samples:** ["https://t.co/ZeUZL6Khxn"]

### `dmConversation.messages[0].messageCreate.urls[4].display`
- **Types:** {"String"}
- **Optional:** true (1/1000 items)
- **Samples:** ["wa.me/9980020475"]

### `dmConversation.messages[0].messageCreate.urls[4].expanded`
- **Types:** {"String"}
- **Optional:** true (1/1000 items)
- **Samples:** ["http://wa.me/9980020475"]

### `dmConversation.messages[0].messageCreate.urls[4].url`
- **Types:** {"String"}
- **Optional:** true (1/1000 items)
- **Samples:** ["https://t.co/jP6ubgx3do"]

### `dmConversation.messages[1].messageCreate`
- **Types:** {"Object"}
- **Optional:** true (901/1000 items)
- **Samples:** ["{9 fields}", "{9 fields}", "{9 fields}"]

### `dmConversation.messages[1].messageCreate.createdAt`
- **Types:** {"String"}
- **Optional:** true (901/1000 items)
- **Samples:** ["2025-05-12T03:33:49.162Z", "2025-01-15T16:59:09.660Z", "2022-01-31T13:24:23.354Z"]

### `dmConversation.messages[1].messageCreate.editHistory`
- **Types:** {"Array"}
- **Optional:** true (899/1000 items)
- **Samples:** ["[0 items]", "[0 items]", "[0 items]"]

### `dmConversation.messages[1].messageCreate.id`
- **Types:** {"String"}
- **Optional:** true (901/1000 items)
- **Samples:** ["1921770762052173967", "1879574052483809535", "1488141108597166085"]

### `dmConversation.messages[1].messageCreate.mediaUrls`
- **Types:** {"Array"}
- **Optional:** true (901/1000 items)
- **Samples:** ["[0 items]", "[0 items]", "[0 items]"]

### `dmConversation.messages[1].messageCreate.reactions`
- **Types:** {"Array"}
- **Optional:** true (901/1000 items)
- **Samples:** ["[1 items]", "[0 items]", "[1 items]"]

### `dmConversation.messages[1].messageCreate.reactions[0].createdAt`
- **Types:** {"String"}
- **Optional:** true (182/1000 items)
- **Samples:** ["2025-05-12T03:36:46.408Z", "2022-01-31T13:24:58.003Z", "2022-03-02T11:25:02.891Z"]

### `dmConversation.messages[1].messageCreate.reactions[0].eventId`
- **Types:** {"String"}
- **Optional:** true (182/1000 items)
- **Samples:** ["1921771505672978432", "1488141254227787786", "1498982711641673728"]

### `dmConversation.messages[1].messageCreate.reactions[0].reactionKey`
- **Types:** {"String"}
- **Optional:** true (182/1000 items)
- **Samples:** ["excited", "excited", "agree"]

### `dmConversation.messages[1].messageCreate.reactions[0].senderId`
- **Types:** {"String"}
- **Optional:** true (182/1000 items)
- **Samples:** ["3382", "1132151165410455552", "274332925"]

### `dmConversation.messages[1].messageCreate.recipientId`
- **Types:** {"String"}
- **Optional:** true (901/1000 items)
- **Samples:** ["3382", "3977", "1132151165410455552"]

### `dmConversation.messages[1].messageCreate.senderId`
- **Types:** {"String"}
- **Optional:** true (901/1000 items)
- **Samples:** ["1132151165410455552", "1132151165410455552", "1315501"]

### `dmConversation.messages[1].messageCreate.text`
- **Types:** {"String"}
- **Optional:** true (901/1000 items)
- **Samples:** ["Just pure text predictions where your thoughts flo", "Really cool stuff you build", "Thanks for your contribution to this 🙏🏻\n\nhttps://t"]

### `dmConversation.messages[1].messageCreate.urls`
- **Types:** {"Array"}
- **Optional:** true (901/1000 items)
- **Samples:** ["[0 items]", "[0 items]", "[1 items]"]

### `dmConversation.messages[1].messageCreate.urls[0].display`
- **Types:** {"String"}
- **Optional:** true (132/1000 items)
- **Samples:** ["twitter.com/kpfrahm/status…", "forms.gle/CeJKavNQ1DGXqU…", "twitter.com/amuldotexe/sta…"]

### `dmConversation.messages[1].messageCreate.urls[0].expanded`
- **Types:** {"String"}
- **Optional:** true (132/1000 items)
- **Samples:** ["https://twitter.com/kpfrahm/status/148812905314406", "https://forms.gle/CeJKavNQ1DGXqUE17", "https://twitter.com/amuldotexe/status/171556164987"]

### `dmConversation.messages[1].messageCreate.urls[0].url`
- **Types:** {"String"}
- **Optional:** true (132/1000 items)
- **Samples:** ["https://t.co/DngfsjCLyf", "https://t.co/84VZUG27ZX", "https://t.co/KjDwPAW8AW"]

### `dmConversation.messages[1].messageCreate.urls[1].display`
- **Types:** {"String"}
- **Optional:** true (23/1000 items)
- **Samples:** ["insightful.in", "twitter.com/yashmiadani/st…", "wa.me/9886248882"]

### `dmConversation.messages[1].messageCreate.urls[1].expanded`
- **Types:** {"String"}
- **Optional:** true (23/1000 items)
- **Samples:** ["http://insightful.in", "https://twitter.com/yashmiadani/status/19270396980", "http://wa.me/9886248882"]

### `dmConversation.messages[1].messageCreate.urls[1].url`
- **Types:** {"String"}
- **Optional:** true (23/1000 items)
- **Samples:** ["https://t.co/ZeUZL6Khxn", "https://t.co/SR04PfoaLu", "https://t.co/zQ3TU8CRJS"]

### `dmConversation.messages[1].messageCreate.urls[2].display`
- **Types:** {"String"}
- **Optional:** true (4/1000 items)
- **Samples:** ["g.co/kgs/AyGmss", "g.co/kgs/AyGmss", "g.co/kgs/AyGmss"]

### `dmConversation.messages[1].messageCreate.urls[2].expanded`
- **Types:** {"String"}
- **Optional:** true (4/1000 items)
- **Samples:** ["https://g.co/kgs/AyGmss", "https://g.co/kgs/AyGmss", "https://g.co/kgs/AyGmss"]

### `dmConversation.messages[1].messageCreate.urls[2].url`
- **Types:** {"String"}
- **Optional:** true (4/1000 items)
- **Samples:** ["https://t.co/Yty7bN9VlI", "https://t.co/Yty7bN9VlI", "https://t.co/Yty7bN9nwa"]

### `dmConversation.messages[1].messageCreate.urls[3].display`
- **Types:** {"String"}
- **Optional:** true (4/1000 items)
- **Samples:** ["insightful.in", "insightful.in", "insightful.in"]

### `dmConversation.messages[1].messageCreate.urls[3].expanded`
- **Types:** {"String"}
- **Optional:** true (4/1000 items)
- **Samples:** ["http://insightful.in", "http://insightful.in", "http://insightful.in"]

### `dmConversation.messages[1].messageCreate.urls[3].url`
- **Types:** {"String"}
- **Optional:** true (4/1000 items)
- **Samples:** ["https://t.co/ZeUZL6KPmV", "https://t.co/ZeUZL6KPmV", "https://t.co/ZeUZL6Khxn"]

### `dmConversation.messages[1].messageCreate.urls[4].display`
- **Types:** {"String"}
- **Optional:** true (2/1000 items)
- **Samples:** ["wa.me/9980020475", "wa.me/9980020475"]

### `dmConversation.messages[1].messageCreate.urls[4].expanded`
- **Types:** {"String"}
- **Optional:** true (2/1000 items)
- **Samples:** ["http://wa.me/9980020475", "http://wa.me/9980020475"]

### `dmConversation.messages[1].messageCreate.urls[4].url`
- **Types:** {"String"}
- **Optional:** true (2/1000 items)
- **Samples:** ["https://t.co/jP6ubgxB2W", "https://t.co/jP6ubgx3do"]

### `dmConversation.messages[2].messageCreate`
- **Types:** {"Object"}
- **Optional:** true (816/1000 items)
- **Samples:** ["{9 fields}", "{9 fields}", "{9 fields}"]

### `dmConversation.messages[2].messageCreate.createdAt`
- **Types:** {"String"}
- **Optional:** true (816/1000 items)
- **Samples:** ["2025-05-12T03:33:06.818Z", "2023-01-22T14:39:36.823Z", "2023-11-29T08:28:35.127Z"]

### `dmConversation.messages[2].messageCreate.editHistory`
- **Types:** {"Array"}
- **Optional:** true (816/1000 items)
- **Samples:** ["[0 items]", "[0 items]", "[0 items]"]

### `dmConversation.messages[2].messageCreate.id`
- **Types:** {"String"}
- **Optional:** true (816/1000 items)
- **Samples:** ["1921770584486346796", "1617170119809200133", "1729779373707002260"]

### `dmConversation.messages[2].messageCreate.mediaUrls`
- **Types:** {"Array"}
- **Optional:** true (816/1000 items)
- **Samples:** ["[0 items]", "[0 items]", "[0 items]"]

### `dmConversation.messages[2].messageCreate.reactions`
- **Types:** {"Array"}
- **Optional:** true (816/1000 items)
- **Samples:** ["[0 items]", "[0 items]", "[0 items]"]

### `dmConversation.messages[2].messageCreate.reactions[0].createdAt`
- **Types:** {"String"}
- **Optional:** true (137/1000 items)
- **Samples:** ["2022-12-19T13:40:57.084Z", "2022-07-09T05:20:27.391Z", "2023-12-26T05:31:36.806Z"]

### `dmConversation.messages[2].messageCreate.reactions[0].eventId`
- **Types:** {"String"}
- **Optional:** true (137/1000 items)
- **Samples:** ["1604834169636343808", "1545638993890783232", "1739519309700026368"]

### `dmConversation.messages[2].messageCreate.reactions[0].reactionKey`
- **Types:** {"String"}
- **Optional:** true (137/1000 items)
- **Samples:** ["agree", "agree", "like"]

### `dmConversation.messages[2].messageCreate.reactions[0].senderId`
- **Types:** {"String"}
- **Optional:** true (137/1000 items)
- **Samples:** ["270393273", "1132151165410455552", "1132151165410455552"]

### `dmConversation.messages[2].messageCreate.recipientId`
- **Types:** {"String"}
- **Optional:** true (816/1000 items)
- **Samples:** ["3382", "15096544", "262622456"]

### `dmConversation.messages[2].messageCreate.senderId`
- **Types:** {"String"}
- **Optional:** true (816/1000 items)
- **Samples:** ["1132151165410455552", "1132151165410455552", "1132151165410455552"]

### `dmConversation.messages[2].messageCreate.text`
- **Types:** {"String"}
- **Optional:** true (816/1000 items)
- **Samples:** ["https://t.co/IOivmVgaCe\n\nI should probably do a vi", "So not attacking new personal problems", "Thak gya hun bro"]

### `dmConversation.messages[2].messageCreate.urls`
- **Types:** {"Array"}
- **Optional:** true (816/1000 items)
- **Samples:** ["[2 items]", "[0 items]", "[0 items]"]

### `dmConversation.messages[2].messageCreate.urls[0].display`
- **Types:** {"String"}
- **Optional:** true (94/1000 items)
- **Samples:** ["x.com/amuldotexe/sta…", "linkedin.com/jobs/view/3160…", "twitter.com/amuldotexe/sta…"]

### `dmConversation.messages[2].messageCreate.urls[0].expanded`
- **Types:** {"String"}
- **Optional:** true (94/1000 items)
- **Samples:** ["https://x.com/amuldotexe/status/192176273702302957", "https://www.linkedin.com/jobs/view/3160460225/?cap", "https://twitter.com/amuldotexe/status/168558004602"]

### `dmConversation.messages[2].messageCreate.urls[0].url`
- **Types:** {"String"}
- **Optional:** true (94/1000 items)
- **Samples:** ["https://t.co/IOivmVgaCe", "https://t.co/ST5m8iJmV9", "https://t.co/UoNci3svZm"]

### `dmConversation.messages[2].messageCreate.urls[1].display`
- **Types:** {"String"}
- **Optional:** true (8/1000 items)
- **Samples:** ["twitter.com/amuldotexe/sta…", "twitter.com/yashmiadani/st…", "twitter.com/fooobar/status…"]

### `dmConversation.messages[2].messageCreate.urls[1].expanded`
- **Types:** {"String"}
- **Optional:** true (8/1000 items)
- **Samples:** ["https://twitter.com/amuldotexe/status/192176273702", "https://twitter.com/yashmiadani/status/19008467593", "https://twitter.com/fooobar/status/191495234928769"]

### `dmConversation.messages[2].messageCreate.urls[1].url`
- **Types:** {"String"}
- **Optional:** true (8/1000 items)
- **Samples:** ["https://t.co/npODyffYqC", "https://t.co/Sf07sgX2aF", "https://t.co/LndCeFzFrm"]

### `dmConversation.messages[2].messageCreate.urls[2].display`
- **Types:** {"String"}
- **Optional:** true (1/1000 items)
- **Samples:** ["twitter.com/amuldotexe/sta…"]

### `dmConversation.messages[2].messageCreate.urls[2].expanded`
- **Types:** {"String"}
- **Optional:** true (1/1000 items)
- **Samples:** ["https://twitter.com/amuldotexe/status/182681935289"]

### `dmConversation.messages[2].messageCreate.urls[2].url`
- **Types:** {"String"}
- **Optional:** true (1/1000 items)
- **Samples:** ["https://t.co/ML9oV671NV"]

### `dmConversation.messages[3].messageCreate`
- **Types:** {"Object"}
- **Optional:** true (737/1000 items)
- **Samples:** ["{9 fields}", "{9 fields}", "{9 fields}"]

### `dmConversation.messages[3].messageCreate.createdAt`
- **Types:** {"String"}
- **Optional:** true (737/1000 items)
- **Samples:** ["2025-04-28T15:17:15.577Z", "2023-01-22T14:39:27.480Z", "2023-11-29T08:28:30.965Z"]

### `dmConversation.messages[3].messageCreate.editHistory`
- **Types:** {"Array"}
- **Optional:** true (737/1000 items)
- **Samples:** ["[0 items]", "[0 items]", "[0 items]"]

### `dmConversation.messages[3].messageCreate.editHistory[0].createdAtSec`
- **Types:** {"String"}
- **Optional:** true (1/1000 items)
- **Samples:** ["1744529501"]

### `dmConversation.messages[3].messageCreate.editHistory[0].editedText`
- **Types:** {"String"}
- **Optional:** true (1/1000 items)
- **Samples:** ["Amul  sirji ,congratulations on your new home!\nI’v"]

### `dmConversation.messages[3].messageCreate.id`
- **Types:** {"String"}
- **Optional:** true (737/1000 items)
- **Samples:** ["1916874358494040337", "1617170080621821956", "1729779356254482501"]

### `dmConversation.messages[3].messageCreate.mediaUrls`
- **Types:** {"Array"}
- **Optional:** true (737/1000 items)
- **Samples:** ["[0 items]", "[0 items]", "[0 items]"]

### `dmConversation.messages[3].messageCreate.reactions`
- **Types:** {"Array"}
- **Optional:** true (737/1000 items)
- **Samples:** ["[0 items]", "[0 items]", "[0 items]"]

### `dmConversation.messages[3].messageCreate.reactions[0].createdAt`
- **Types:** {"String"}
- **Optional:** true (132/1000 items)
- **Samples:** ["2022-12-19T13:35:20.108Z", "2023-09-14T04:41:37.566Z", "2025-03-17T13:55:21.526Z"]

### `dmConversation.messages[3].messageCreate.reactions[0].eventId`
- **Types:** {"String"}
- **Optional:** true (132/1000 items)
- **Samples:** ["1604832756248125440", "1702180779802173440", "1901633457249529857"]

### `dmConversation.messages[3].messageCreate.reactions[0].reactionKey`
- **Types:** {"String"}
- **Optional:** true (132/1000 items)
- **Samples:** ["excited", "agree", "like"]

### `dmConversation.messages[3].messageCreate.reactions[0].senderId`
- **Types:** {"String"}
- **Optional:** true (132/1000 items)
- **Samples:** ["1132151165410455552", "1132151165410455552", "1132151165410455552"]

### `dmConversation.messages[3].messageCreate.recipientId`
- **Types:** {"String"}
- **Optional:** true (737/1000 items)
- **Samples:** ["3382", "15096544", "262622456"]

### `dmConversation.messages[3].messageCreate.senderId`
- **Types:** {"String"}
- **Optional:** true (737/1000 items)
- **Samples:** ["1132151165410455552", "1132151165410455552", "1132151165410455552"]

### `dmConversation.messages[3].messageCreate.text`
- **Types:** {"String"}
- **Optional:** true (737/1000 items)
- **Samples:** ["but there's gotta be a more mature graceful low-dr", "I just moved to a new team", "Kitna content dekhun"]

### `dmConversation.messages[3].messageCreate.urls`
- **Types:** {"Array"}
- **Optional:** true (737/1000 items)
- **Samples:** ["[0 items]", "[0 items]", "[0 items]"]

### `dmConversation.messages[3].messageCreate.urls[0].display`
- **Types:** {"String"}
- **Optional:** true (62/1000 items)
- **Samples:** ["pic.twitter.com/xRY8WMTS5z", "g.co/kgs/AyGmss", "twitter.com/amuldotexe/sta…"]

### `dmConversation.messages[3].messageCreate.urls[0].expanded`
- **Types:** {"String"}
- **Optional:** true (62/1000 items)
- **Samples:** ["https://twitter.com/messages/media/187885601704556", "https://g.co/kgs/AyGmss", "https://twitter.com/amuldotexe/status/152113439539"]

### `dmConversation.messages[3].messageCreate.urls[0].url`
- **Types:** {"String"}
- **Optional:** true (62/1000 items)
- **Samples:** ["https://t.co/xRY8WMTS5z", "https://t.co/Yty7bN9nwa", "https://t.co/wkZCnTOntP"]

### `dmConversation.messages[3].messageCreate.urls[1].display`
- **Types:** {"String"}
- **Optional:** true (10/1000 items)
- **Samples:** ["insightful.in", "twitter.com/amuldotexe/sta…", "twitter.com/amuldotexe/sta…"]

### `dmConversation.messages[3].messageCreate.urls[1].expanded`
- **Types:** {"String"}
- **Optional:** true (10/1000 items)
- **Samples:** ["http://insightful.in", "https://twitter.com/amuldotexe/status/192303490421", "https://twitter.com/amuldotexe/status/155249731861"]

### `dmConversation.messages[3].messageCreate.urls[1].url`
- **Types:** {"String"}
- **Optional:** true (10/1000 items)
- **Samples:** ["https://t.co/ZeUZL6Khxn", "https://t.co/A8dZeQX3Fm", "https://t.co/IucvSRRCtE"]

### `dmConversation.messages[4].messageCreate`
- **Types:** {"Object"}
- **Optional:** true (669/1000 items)
- **Samples:** ["{9 fields}", "{9 fields}", "{9 fields}"]

### `dmConversation.messages[4].messageCreate.createdAt`
- **Types:** {"String"}
- **Optional:** true (669/1000 items)
- **Samples:** ["2025-04-28T15:15:40.548Z", "2023-01-22T14:39:18.076Z", "2023-11-29T08:28:26.310Z"]

### `dmConversation.messages[4].messageCreate.editHistory`
- **Types:** {"Array"}
- **Optional:** true (669/1000 items)
- **Samples:** ["[0 items]", "[0 items]", "[0 items]"]

### `dmConversation.messages[4].messageCreate.editHistory[0].createdAtSec`
- **Types:** {"String"}
- **Optional:** true (1/1000 items)
- **Samples:** ["1738820628"]

### `dmConversation.messages[4].messageCreate.editHistory[0].editedText`
- **Types:** {"String"}
- **Optional:** true (1/1000 items)
- **Samples:** ["Hi Amul,\nComing here from your being near Manyata."]

### `dmConversation.messages[4].messageCreate.editHistory[1].createdAtSec`
- **Types:** {"String"}
- **Optional:** true (1/1000 items)
- **Samples:** ["0"]

### `dmConversation.messages[4].messageCreate.editHistory[1].editedText`
- **Types:** {"String"}
- **Optional:** true (1/1000 items)
- **Samples:** ["Hi Amul,\nComing here from your today's tweet, abou"]

### `dmConversation.messages[4].messageCreate.id`
- **Types:** {"String"}
- **Optional:** true (669/1000 items)
- **Samples:** ["1916873959955677303", "1617170041212145669", "1729779336713282013"]

### `dmConversation.messages[4].messageCreate.mediaUrls`
- **Types:** {"Array"}
- **Optional:** true (669/1000 items)
- **Samples:** ["[0 items]", "[0 items]", "[0 items]"]

### `dmConversation.messages[4].messageCreate.reactions`
- **Types:** {"Array"}
- **Optional:** true (669/1000 items)
- **Samples:** ["[0 items]", "[0 items]", "[0 items]"]

### `dmConversation.messages[4].messageCreate.reactions[0].createdAt`
- **Types:** {"String"}
- **Optional:** true (135/1000 items)
- **Samples:** ["2023-06-18T05:43:42.154Z", "2024-09-14T05:44:47.264Z", "2022-05-16T17:37:32.746Z"]

### `dmConversation.messages[4].messageCreate.reactions[0].eventId`
- **Types:** {"String"}
- **Optional:** true (135/1000 items)
- **Samples:** ["1670306269704626176", "1834830633769549825", "1526255543706210305"]

### `dmConversation.messages[4].messageCreate.reactions[0].reactionKey`
- **Types:** {"String"}
- **Optional:** true (135/1000 items)
- **Samples:** ["agree", "like", "excited"]

### `dmConversation.messages[4].messageCreate.reactions[0].senderId`
- **Types:** {"String"}
- **Optional:** true (135/1000 items)
- **Samples:** ["269917986", "1132151165410455552", "1132151165410455552"]

### `dmConversation.messages[4].messageCreate.recipientId`
- **Types:** {"String"}
- **Optional:** true (669/1000 items)
- **Samples:** ["3382", "15096544", "262622456"]

### `dmConversation.messages[4].messageCreate.senderId`
- **Types:** {"String"}
- **Optional:** true (669/1000 items)
- **Samples:** ["1132151165410455552", "1132151165410455552", "1132151165410455552"]

### `dmConversation.messages[4].messageCreate.text`
- **Types:** {"String"}
- **Optional:** true (669/1000 items)
- **Samples:** ["Currently ideating with friends to build small ind", "Interesting", "Okis"]

### `dmConversation.messages[4].messageCreate.urls`
- **Types:** {"Array"}
- **Optional:** true (669/1000 items)
- **Samples:** ["[0 items]", "[0 items]", "[0 items]"]

### `dmConversation.messages[4].messageCreate.urls[0].display`
- **Types:** {"String"}
- **Optional:** true (59/1000 items)
- **Samples:** ["x.com/amuldotexe/sta…", "drive.google.com/file/d/1JMYWdX…", "twitter.com/amuldotexe/sta…"]

### `dmConversation.messages[4].messageCreate.urls[0].expanded`
- **Types:** {"String"}
- **Optional:** true (59/1000 items)
- **Samples:** ["https://x.com/amuldotexe/status/190161210231565561", "https://drive.google.com/file/d/1JMYWdXXWpFDh0t2-P", "https://twitter.com/amuldotexe/status/149897720361"]

### `dmConversation.messages[4].messageCreate.urls[0].url`
- **Types:** {"String"}
- **Optional:** true (59/1000 items)
- **Samples:** ["https://t.co/Wi1nh6n8fs", "https://t.co/0BwQPyJct5", "https://t.co/Cix9GCzocC"]

### `dmConversation.messages[4].messageCreate.urls[1].display`
- **Types:** {"String"}
- **Optional:** true (13/1000 items)
- **Samples:** ["twitter.com/amuldotexe/sta…", "insightful.in", "twitter.com/thekaipullai/s…"]

### `dmConversation.messages[4].messageCreate.urls[1].expanded`
- **Types:** {"String"}
- **Optional:** true (13/1000 items)
- **Samples:** ["https://twitter.com/amuldotexe/status/182694417170", "http://insightful.in", "https://twitter.com/thekaipullai/status/1926860566"]

### `dmConversation.messages[4].messageCreate.urls[1].url`
- **Types:** {"String"}
- **Optional:** true (13/1000 items)
- **Samples:** ["https://t.co/3W8BhbhuUW", "https://t.co/ZeUZL6KPmV", "https://t.co/vUFyRyJJps"]

