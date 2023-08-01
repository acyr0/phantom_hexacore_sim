# results

## description
There are 2 sets of results presented here, you should make your choice based
on which scenario best fits the content you want to complete after New Age.

You can also run the code in this repo yourself, if you want to customize the
results given.

### dummy ba
[![dummy ba, page 1](img/thumb/dummy_ba_1.png)](img/dummy_ba_1.png)
[![dummy ba, page 2](img/thumb/dummy_ba_2.png)](img/dummy_ba_2.png)
[![dummy ba, page 3](img/thumb/dummy_ba_3.png)](img/dummy_ba_3.png)

This is a dummy BA, but not a standard 2:50 DPM BA. Instead, it is fully
buffed, and ~9 minutes with 4 bursts, starting and ending on a burst
(effectively, a 12 min DPM BA, but intentionally missing the last 3 minutes of
DPM).

The general idea here is that, frequently, your bosses will start and end on a
burst. Therefore, we should not be trying to optimize for a BA that ends right
before the burst comes back up, as that will deflate the contribution of our
front-loaded burst more than would realistically occur in bosses. 9 minutes was
arbitrarily chosen here (rather than something like 3 minutes) to avoid skewing
the results too far towards the burst skills.

You should use this if you want to optimize for solo, high-Tempest-uptime
bosses after New Age.

### hard seren p2 ft. bishop + kanna
[![hard seren p2 ba ft. bishop + kanna, page 1](img/thumb/hseren_ba_1.png)](img/hseren_ba_1.png)
[![hard seren p2 ba ft. bishop + kanna, page 2](img/thumb/hseren_ba_2.png)](img/hseren_ba_2.png)

This is a 4.5 minute, 2 burst Hard Seren P2 BA (roughly equivalent to the 9
minute 4 burst dummy BA above). Thanks to mostly Benediction, this skews the BA
contribution very far in favor of Luck of the Draw. Seren is also a boss where
Tempest uptime is difficult.

You should use this if you want to optimize for party, low-Tempest-uptime
bosses after New Age.

## sol erda fragments are the bottleneck

### based on a dummy ba
```
Cost: 0        FD Gain: 12.66%    mFD/cost:            Origin: 1     Mastery: 0     LotD: 0     Ace: 0     Mark: 0     Rift Break: 0
Cost: 75       FD Gain: 14.58%    mFD/cost:  25.34%    Origin: 1     Mastery: 0     LotD: 0     Ace: 1     Mark: 0     Rift Break: 0
Cost: 150      FD Gain: 16.40%    mFD/cost:  24.01%    Origin: 1     Mastery: 0     LotD: 1     Ace: 1     Mark: 0     Rift Break: 0
Cost: 180      FD Gain: 16.81%    mFD/cost:  13.59%    Origin: 2     Mastery: 0     LotD: 1     Ace: 1     Mark: 0     Rift Break: 0
Cost: 255      FD Gain: 17.70%    mFD/cost:  11.84%    Origin: 2     Mastery: 0     LotD: 1     Ace: 1     Mark: 1     Rift Break: 0
Cost: 290      FD Gain: 18.11%    mFD/cost:  11.65%    Origin: 3     Mastery: 0     LotD: 1     Ace: 1     Mark: 1     Rift Break: 0
Cost: 532      FD Gain: 20.82%    mFD/cost:  11.03%    Origin: 3     Mastery: 9     LotD: 1     Ace: 1     Mark: 1     Rift Break: 0
Cost: 617      FD Gain: 21.63%    mFD/cost:   9.57%    Origin: 5     Mastery: 9     LotD: 1     Ace: 1     Mark: 1     Rift Break: 0
Cost: 692      FD Gain: 22.30%    mFD/cost:   8.86%    Origin: 5     Mastery: 9     LotD: 1     Ace: 1     Mark: 1     Rift Break: 1
Cost: 742      FD Gain: 22.71%    mFD/cost:   8.15%    Origin: 6     Mastery: 9     LotD: 1     Ace: 1     Mark: 1     Rift Break: 1
Cost: 765      FD Gain: 22.88%    mFD/cost:   7.58%    Origin: 6     Mastery: 9     LotD: 1     Ace: 2     Mark: 1     Rift Break: 1
Cost: 820      FD Gain: 23.29%    mFD/cost:   7.41%    Origin: 7     Mastery: 9     LotD: 1     Ace: 2     Mark: 1     Rift Break: 1
Cost: 843      FD Gain: 23.46%    mFD/cost:   7.18%    Origin: 7     Mastery: 9     LotD: 2     Ace: 2     Mark: 1     Rift Break: 1
Cost: 903      FD Gain: 23.86%    mFD/cost:   6.80%    Origin: 8     Mastery: 9     LotD: 2     Ace: 2     Mark: 1     Rift Break: 1
Cost: 930      FD Gain: 24.04%    mFD/cost:   6.45%    Origin: 8     Mastery: 9     LotD: 2     Ace: 3     Mark: 1     Rift Break: 1
Cost: 995      FD Gain: 24.45%    mFD/cost:   6.27%    Origin: 9     Mastery: 9     LotD: 2     Ace: 3     Mark: 1     Rift Break: 1
Cost: 1022     FD Gain: 24.61%    mFD/cost:   6.11%    Origin: 9     Mastery: 9     LotD: 3     Ace: 3     Mark: 1     Rift Break: 1
Cost: 1312     FD Gain: 26.34%    mFD/cost:   5.91%    Origin: 9     Mastery: 14    LotD: 3     Ace: 3     Mark: 1     Rift Break: 1
Cost: 1342     FD Gain: 26.51%    mFD/cost:   5.81%    Origin: 9     Mastery: 14    LotD: 3     Ace: 4     Mark: 1     Rift Break: 1
Cost: 1402     FD Gain: 26.86%    mFD/cost:   5.75%    Origin: 9     Mastery: 15    LotD: 3     Ace: 4     Mark: 1     Rift Break: 1
Cost: 1432     FD Gain: 27.03%    mFD/cost:   5.50%    Origin: 9     Mastery: 15    LotD: 4     Ace: 4     Mark: 1     Rift Break: 1
Cost: 1790     FD Gain: 28.94%    mFD/cost:   5.31%    Origin: 9     Mastery: 15    LotD: 4     Ace: 10    Mark: 1     Rift Break: 1
Cost: 1855     FD Gain: 29.29%    mFD/cost:   5.31%    Origin: 9     Mastery: 16    LotD: 4     Ace: 10    Mark: 1     Rift Break: 1
Cost: 2213     FD Gain: 31.11%    mFD/cost:   5.03%    Origin: 9     Mastery: 16    LotD: 10    Ace: 10    Mark: 1     Rift Break: 1
Cost: 2438     FD Gain: 32.14%    mFD/cost:   4.58%    Origin: 9     Mastery: 19    LotD: 10    Ace: 10    Mark: 1     Rift Break: 1
Cost: 3018     FD Gain: 34.39%    mFD/cost:   3.83%    Origin: 14    Mastery: 19    LotD: 10    Ace: 10    Mark: 1     Rift Break: 1
Cost: 3041     FD Gain: 34.47%    mFD/cost:   3.52%    Origin: 14    Mastery: 19    LotD: 10    Ace: 10    Mark: 2     Rift Break: 1
Cost: 3291     FD Gain: 35.30%    mFD/cost:   3.29%    Origin: 16    Mastery: 19    LotD: 10    Ace: 10    Mark: 2     Rift Break: 1
Cost: 4051     FD Gain: 37.72%    mFD/cost:   3.14%    Origin: 16    Mastery: 26    LotD: 10    Ace: 10    Mark: 2     Rift Break: 1
Cost: 4078     FD Gain: 37.80%    mFD/cost:   3.00%    Origin: 16    Mastery: 26    LotD: 10    Ace: 10    Mark: 3     Rift Break: 1
Cost: 4193     FD Gain: 38.14%    mFD/cost:   3.00%    Origin: 16    Mastery: 27    LotD: 10    Ace: 10    Mark: 3     Rift Break: 1
Cost: 4333     FD Gain: 38.56%    mFD/cost:   2.95%    Origin: 17    Mastery: 27    LotD: 10    Ace: 10    Mark: 3     Rift Break: 1
Cost: 4393     FD Gain: 38.73%    mFD/cost:   2.90%    Origin: 17    Mastery: 27    LotD: 10    Ace: 11    Mark: 3     Rift Break: 1
Cost: 4638     FD Gain: 39.42%    mFD/cost:   2.81%    Origin: 17    Mastery: 29    LotD: 10    Ace: 11    Mark: 3     Rift Break: 1
Cost: 4698     FD Gain: 39.59%    mFD/cost:   2.75%    Origin: 17    Mastery: 29    LotD: 11    Ace: 11    Mark: 3     Rift Break: 1
Cost: 4848     FD Gain: 40.00%    mFD/cost:   2.75%    Origin: 18    Mastery: 29    LotD: 11    Ace: 11    Mark: 3     Rift Break: 1
Cost: 4878     FD Gain: 40.08%    mFD/cost:   2.70%    Origin: 18    Mastery: 29    LotD: 11    Ace: 11    Mark: 4     Rift Break: 1
Cost: 4901     FD Gain: 40.14%    mFD/cost:   2.63%    Origin: 18    Mastery: 29    LotD: 11    Ace: 11    Mark: 4     Rift Break: 2
Cost: 5061     FD Gain: 40.56%    mFD/cost:   2.58%    Origin: 19    Mastery: 29    LotD: 11    Ace: 11    Mark: 4     Rift Break: 2
Cost: 5129     FD Gain: 40.73%    mFD/cost:   2.56%    Origin: 19    Mastery: 29    LotD: 11    Ace: 12    Mark: 4     Rift Break: 2
Cost: 5649     FD Gain: 42.05%    mFD/cost:   2.53%    Origin: 21    Mastery: 29    LotD: 11    Ace: 12    Mark: 4     Rift Break: 2
Cost: 6007     FD Gain: 42.94%    mFD/cost:   2.48%    Origin: 21    Mastery: 29    LotD: 11    Ace: 12    Mark: 10    Rift Break: 2
Cost: 6075     FD Gain: 43.11%    mFD/cost:   2.43%    Origin: 21    Mastery: 29    LotD: 12    Ace: 12    Mark: 10    Rift Break: 2
Cost: 7022     FD Gain: 45.38%    mFD/cost:   2.37%    Origin: 21    Mastery: 29    LotD: 12    Ace: 20    Mark: 10    Rift Break: 2
Cost: 9242     FD Gain: 50.50%    mFD/cost:   2.25%    Origin: 30    Mastery: 29    LotD: 12    Ace: 20    Mark: 10    Rift Break: 2
Cost: 10189    FD Gain: 52.65%    mFD/cost:   2.24%    Origin: 30    Mastery: 29    LotD: 20    Ace: 20    Mark: 10    Rift Break: 2
Cost: 10246    FD Gain: 52.77%    mFD/cost:   2.13%    Origin: 30    Mastery: 29    LotD: 20    Ace: 20    Mark: 10    Rift Break: 4
Cost: 12041    FD Gain: 56.26%    mFD/cost:   1.91%    Origin: 30    Mastery: 29    LotD: 20    Ace: 30    Mark: 10    Rift Break: 4
Cost: 12399    FD Gain: 56.93%    mFD/cost:   1.86%    Origin: 30    Mastery: 29    LotD: 20    Ace: 30    Mark: 10    Rift Break: 10
Cost: 14194    FD Gain: 60.23%    mFD/cost:   1.81%    Origin: 30    Mastery: 29    LotD: 30    Ace: 30    Mark: 10    Rift Break: 10
Cost: 14444    FD Gain: 60.58%    mFD/cost:   1.38%    Origin: 30    Mastery: 30    LotD: 30    Ace: 30    Mark: 10    Rift Break: 10
Cost: 15519    FD Gain: 61.79%    mFD/cost:   1.12%    Origin: 30    Mastery: 30    LotD: 30    Ace: 30    Mark: 20    Rift Break: 10
Cost: 15579    FD Gain: 61.85%    mFD/cost:   1.01%    Origin: 30    Mastery: 30    LotD: 30    Ace: 30    Mark: 20    Rift Break: 11
Cost: 17374    FD Gain: 63.48%    mFD/cost:   0.90%    Origin: 30    Mastery: 30    LotD: 30    Ace: 30    Mark: 30    Rift Break: 11
Cost: 20184    FD Gain: 65.54%    mFD/cost:   0.73%    Origin: 30    Mastery: 30    LotD: 30    Ace: 30    Mark: 30    Rift Break: 30
```

### based on hard seren p2 ft. bishop + kanna
```
Cost: 0        FD Gain: 22.95%    mFD/cost:            Origin: 1     Mastery: 0     LotD: 0     Ace: 0     Mark: 0     Rift Break: 0
Cost: 75       FD Gain: 26.24%    mFD/cost:  43.16%    Origin: 1     Mastery: 0     LotD: 1     Ace: 0     Mark: 0     Rift Break: 0
Cost: 105      FD Gain: 26.98%    mFD/cost:  24.59%    Origin: 2     Mastery: 0     LotD: 1     Ace: 0     Mark: 0     Rift Break: 0
Cost: 180      FD Gain: 28.79%    mFD/cost:  23.90%    Origin: 2     Mastery: 0     LotD: 1     Ace: 1     Mark: 0     Rift Break: 0
Cost: 405      FD Gain: 32.49%    mFD/cost:  16.16%    Origin: 7     Mastery: 0     LotD: 1     Ace: 1     Mark: 0     Rift Break: 0
Cost: 428      FD Gain: 32.79%    mFD/cost:  12.98%    Origin: 7     Mastery: 0     LotD: 2     Ace: 1     Mark: 0     Rift Break: 0
Cost: 488      FD Gain: 33.53%    mFD/cost:  12.30%    Origin: 8     Mastery: 0     LotD: 2     Ace: 1     Mark: 0     Rift Break: 0
Cost: 563      FD Gain: 34.42%    mFD/cost:  11.81%    Origin: 8     Mastery: 0     LotD: 2     Ace: 1     Mark: 1     Rift Break: 0
Cost: 628      FD Gain: 35.16%    mFD/cost:  11.35%    Origin: 9     Mastery: 0     LotD: 2     Ace: 1     Mark: 1     Rift Break: 0
Cost: 1043     FD Gain: 39.05%    mFD/cost:   9.19%    Origin: 9     Mastery: 0     LotD: 10    Ace: 1     Mark: 1     Rift Break: 0
Cost: 1118     FD Gain: 39.71%    mFD/cost:   8.73%    Origin: 9     Mastery: 0     LotD: 10    Ace: 1     Mark: 1     Rift Break: 1
Cost: 1141     FD Gain: 39.87%    mFD/cost:   7.14%    Origin: 9     Mastery: 0     LotD: 10    Ace: 2     Mark: 1     Rift Break: 1
Cost: 1841     FD Gain: 44.69%    mFD/cost:   6.73%    Origin: 15    Mastery: 0     LotD: 10    Ace: 2     Mark: 1     Rift Break: 1
Cost: 1868     FD Gain: 44.86%    mFD/cost:   6.08%    Origin: 15    Mastery: 0     LotD: 10    Ace: 3     Mark: 1     Rift Break: 1
Cost: 1998     FD Gain: 45.61%    mFD/cost:   5.74%    Origin: 16    Mastery: 0     LotD: 10    Ace: 3     Mark: 1     Rift Break: 1
Cost: 2240     FD Gain: 46.97%    mFD/cost:   5.59%    Origin: 16    Mastery: 9     LotD: 10    Ace: 3     Mark: 1     Rift Break: 1
Cost: 2270     FD Gain: 47.13%    mFD/cost:   5.48%    Origin: 16    Mastery: 9     LotD: 10    Ace: 4     Mark: 1     Rift Break: 1
Cost: 2410     FD Gain: 47.88%    mFD/cost:   5.33%    Origin: 17    Mastery: 9     LotD: 10    Ace: 4     Mark: 1     Rift Break: 1
Cost: 2768     FD Gain: 49.69%    mFD/cost:   5.01%    Origin: 17    Mastery: 9     LotD: 10    Ace: 10    Mark: 1     Rift Break: 1
Cost: 2828     FD Gain: 49.99%    mFD/cost:   4.98%    Origin: 17    Mastery: 9     LotD: 11    Ace: 10    Mark: 1     Rift Break: 1
Cost: 3138     FD Gain: 51.49%    mFD/cost:   4.80%    Origin: 19    Mastery: 9     LotD: 11    Ace: 10    Mark: 1     Rift Break: 1
Cost: 3206     FD Gain: 51.79%    mFD/cost:   4.39%    Origin: 19    Mastery: 9     LotD: 12    Ace: 10    Mark: 1     Rift Break: 1
Cost: 3906     FD Gain: 54.81%    mFD/cost:   4.26%    Origin: 22    Mastery: 9     LotD: 12    Ace: 10    Mark: 1     Rift Break: 1
Cost: 4853     FD Gain: 58.70%    mFD/cost:   4.03%    Origin: 22    Mastery: 9     LotD: 20    Ace: 10    Mark: 1     Rift Break: 1
Cost: 6893     FD Gain: 66.96%    mFD/cost:   3.89%    Origin: 30    Mastery: 9     LotD: 20    Ace: 10    Mark: 1     Rift Break: 1
Cost: 6916     FD Gain: 67.04%    mFD/cost:   3.52%    Origin: 30    Mastery: 9     LotD: 20    Ace: 10    Mark: 2     Rift Break: 1
Cost: 7206     FD Gain: 68.00%    mFD/cost:   3.29%    Origin: 30    Mastery: 14    LotD: 20    Ace: 10    Mark: 2     Rift Break: 1
Cost: 9001     FD Gain: 73.98%    mFD/cost:   3.24%    Origin: 30    Mastery: 14    LotD: 30    Ace: 10    Mark: 2     Rift Break: 1
Cost: 9061     FD Gain: 74.17%    mFD/cost:   3.19%    Origin: 30    Mastery: 15    LotD: 30    Ace: 10    Mark: 2     Rift Break: 1
Cost: 9088     FD Gain: 74.25%    mFD/cost:   3.00%    Origin: 30    Mastery: 15    LotD: 30    Ace: 10    Mark: 3     Rift Break: 1
Cost: 9153     FD Gain: 74.44%    mFD/cost:   2.94%    Origin: 30    Mastery: 16    LotD: 30    Ace: 10    Mark: 3     Rift Break: 1
Cost: 9213     FD Gain: 74.61%    mFD/cost:   2.74%    Origin: 30    Mastery: 16    LotD: 30    Ace: 11    Mark: 3     Rift Break: 1
Cost: 9283     FD Gain: 74.80%    mFD/cost:   2.73%    Origin: 30    Mastery: 17    LotD: 30    Ace: 11    Mark: 3     Rift Break: 1
Cost: 9313     FD Gain: 74.88%    mFD/cost:   2.70%    Origin: 30    Mastery: 17    LotD: 30    Ace: 11    Mark: 4     Rift Break: 1
Cost: 9336     FD Gain: 74.94%    mFD/cost:   2.59%    Origin: 30    Mastery: 17    LotD: 30    Ace: 11    Mark: 4     Rift Break: 2
Cost: 9411     FD Gain: 75.13%    mFD/cost:   2.55%    Origin: 30    Mastery: 18    LotD: 30    Ace: 11    Mark: 4     Rift Break: 2
Cost: 9769     FD Gain: 76.02%    mFD/cost:   2.47%    Origin: 30    Mastery: 18    LotD: 30    Ace: 11    Mark: 10    Rift Break: 2
Cost: 9837     FD Gain: 76.18%    mFD/cost:   2.42%    Origin: 30    Mastery: 18    LotD: 30    Ace: 12    Mark: 10    Rift Break: 2
Cost: 9917     FD Gain: 76.38%    mFD/cost:   2.39%    Origin: 30    Mastery: 19    LotD: 30    Ace: 12    Mark: 10    Rift Break: 2
Cost: 10864    FD Gain: 78.51%    mFD/cost:   2.23%    Origin: 30    Mastery: 19    LotD: 30    Ace: 20    Mark: 10    Rift Break: 2
Cost: 11279    FD Gain: 79.29%    mFD/cost:   1.86%    Origin: 30    Mastery: 19    LotD: 30    Ace: 20    Mark: 10    Rift Break: 10
Cost: 13074    FD Gain: 82.58%    mFD/cost:   1.80%    Origin: 30    Mastery: 19    LotD: 30    Ace: 30    Mark: 10    Rift Break: 10
Cost: 14194    FD Gain: 84.49%    mFD/cost:   1.69%    Origin: 30    Mastery: 29    LotD: 30    Ace: 30    Mark: 10    Rift Break: 10
Cost: 15269    FD Gain: 85.71%    mFD/cost:   1.12%    Origin: 30    Mastery: 29    LotD: 30    Ace: 30    Mark: 20    Rift Break: 10
Cost: 15329    FD Gain: 85.77%    mFD/cost:   0.99%    Origin: 30    Mastery: 29    LotD: 30    Ace: 30    Mark: 20    Rift Break: 11
Cost: 17124    FD Gain: 87.38%    mFD/cost:   0.89%    Origin: 30    Mastery: 29    LotD: 30    Ace: 30    Mark: 30    Rift Break: 11
Cost: 18139    FD Gain: 88.22%    mFD/cost:   0.82%    Origin: 30    Mastery: 29    LotD: 30    Ace: 30    Mark: 30    Rift Break: 20
Cost: 18389    FD Gain: 88.41%    mFD/cost:   0.77%    Origin: 30    Mastery: 30    LotD: 30    Ace: 30    Mark: 30    Rift Break: 20
Cost: 20184    FD Gain: 89.61%    mFD/cost:   0.66%    Origin: 30    Mastery: 30    LotD: 30    Ace: 30    Mark: 30    Rift Break: 30
```

## sol erda is the bottleneck

### based on a dummy ba
```
Cost: 0        FD Gain: 12.66%    mFD/cost:            Origin: 1     Mastery: 0     LotD: 0     Ace: 0     Mark: 0     Rift Break: 0
Cost: 4        FD Gain: 14.58%    mFD/cost: 476.19%    Origin: 1     Mastery: 0     LotD: 0     Ace: 1     Mark: 0     Rift Break: 0
Cost: 8        FD Gain: 16.40%    mFD/cost: 451.24%    Origin: 1     Mastery: 0     LotD: 1     Ace: 1     Mark: 0     Rift Break: 0
Cost: 11       FD Gain: 17.63%    mFD/cost: 406.89%    Origin: 4     Mastery: 0     LotD: 1     Ace: 1     Mark: 0     Rift Break: 0
Cost: 15       FD Gain: 18.52%    mFD/cost: 222.28%    Origin: 4     Mastery: 0     LotD: 1     Ace: 1     Mark: 1     Rift Break: 0
Cost: 24       FD Gain: 20.53%    mFD/cost: 221.89%    Origin: 4     Mastery: 7     LotD: 1     Ace: 1     Mark: 1     Rift Break: 0
Cost: 30       FD Gain: 21.76%    mFD/cost: 203.24%    Origin: 7     Mastery: 7     LotD: 1     Ace: 1     Mark: 1     Rift Break: 0
Cost: 33       FD Gain: 22.28%    mFD/cost: 174.10%    Origin: 7     Mastery: 7     LotD: 1     Ace: 4     Mark: 1     Rift Break: 0
Cost: 37       FD Gain: 22.97%    mFD/cost: 172.30%    Origin: 7     Mastery: 9     LotD: 1     Ace: 4     Mark: 1     Rift Break: 0
Cost: 41       FD Gain: 23.64%    mFD/cost: 166.24%    Origin: 7     Mastery: 9     LotD: 1     Ace: 4     Mark: 1     Rift Break: 1
Cost: 44       FD Gain: 24.14%    mFD/cost: 164.93%    Origin: 7     Mastery: 9     LotD: 4     Ace: 4     Mark: 1     Rift Break: 1
Cost: 65       FD Gain: 27.24%    mFD/cost: 145.92%    Origin: 7     Mastery: 18    LotD: 4     Ace: 4     Mark: 1     Rift Break: 1
Cost: 71       FD Gain: 28.06%    mFD/cost: 135.72%    Origin: 9     Mastery: 18    LotD: 4     Ace: 4     Mark: 1     Rift Break: 1
Cost: 106      FD Gain: 31.52%    mFD/cost:  97.09%    Origin: 9     Mastery: 28    LotD: 4     Ace: 4     Mark: 1     Rift Break: 1
Cost: 146      FD Gain: 35.42%    mFD/cost:  95.73%    Origin: 18    Mastery: 28    LotD: 4     Ace: 4     Mark: 1     Rift Break: 1
Cost: 166      FD Gain: 37.34%    mFD/cost:  95.06%    Origin: 18    Mastery: 28    LotD: 4     Ace: 10    Mark: 1     Rift Break: 1
Cost: 186      FD Gain: 39.15%    mFD/cost:  90.08%    Origin: 18    Mastery: 28    LotD: 10    Ace: 10    Mark: 1     Rift Break: 1
Cost: 190      FD Gain: 39.50%    mFD/cost:  86.26%    Origin: 18    Mastery: 29    LotD: 10    Ace: 10    Mark: 1     Rift Break: 1
Cost: 195      FD Gain: 39.91%    mFD/cost:  82.51%    Origin: 19    Mastery: 29    LotD: 10    Ace: 10    Mark: 1     Rift Break: 1
Cost: 198      FD Gain: 40.16%    mFD/cost:  81.03%    Origin: 19    Mastery: 29    LotD: 10    Ace: 10    Mark: 4     Rift Break: 1
Cost: 283      FD Gain: 46.60%    mFD/cost:  73.56%    Origin: 30    Mastery: 29    LotD: 10    Ace: 10    Mark: 4     Rift Break: 1
Cost: 379      FD Gain: 52.71%    mFD/cost:  61.74%    Origin: 30    Mastery: 29    LotD: 10    Ace: 30    Mark: 4     Rift Break: 1
Cost: 419      FD Gain: 55.19%    mFD/cost:  61.21%    Origin: 30    Mastery: 29    LotD: 20    Ace: 30    Mark: 4     Rift Break: 1
Cost: 422      FD Gain: 55.37%    mFD/cost:  60.56%    Origin: 30    Mastery: 29    LotD: 20    Ace: 30    Mark: 4     Rift Break: 4
Cost: 478      FD Gain: 58.67%    mFD/cost:  58.06%    Origin: 30    Mastery: 29    LotD: 30    Ace: 30    Mark: 4     Rift Break: 4
Cost: 498      FD Gain: 59.56%    mFD/cost:  44.42%    Origin: 30    Mastery: 29    LotD: 30    Ace: 30    Mark: 10    Rift Break: 4
Cost: 508      FD Gain: 59.91%    mFD/cost:  34.49%    Origin: 30    Mastery: 30    LotD: 30    Ace: 30    Mark: 10    Rift Break: 4
Cost: 528      FD Gain: 60.58%    mFD/cost:  33.22%    Origin: 30    Mastery: 30    LotD: 30    Ace: 30    Mark: 10    Rift Break: 10
Cost: 624      FD Gain: 63.42%    mFD/cost:  29.16%    Origin: 30    Mastery: 30    LotD: 30    Ace: 30    Mark: 30    Rift Break: 10
Cost: 720      FD Gain: 65.54%    mFD/cost:  21.87%    Origin: 30    Mastery: 30    LotD: 30    Ace: 30    Mark: 30    Rift Break: 30
```

### based hard seren p2 ft. bishop + kanna
```
Cost: 0        FD Gain: 22.95%    mFD/cost:            Origin: 1     Mastery: 0     LotD: 0     Ace: 0     Mark: 0     Rift Break: 0
Cost: 4        FD Gain: 26.24%    mFD/cost: 812.30%    Origin: 1     Mastery: 0     LotD: 1     Ace: 0     Mark: 0     Rift Break: 0
Cost: 7        FD Gain: 28.46%    mFD/cost: 734.98%    Origin: 4     Mastery: 0     LotD: 1     Ace: 0     Mark: 0     Rift Break: 0
Cost: 11       FD Gain: 30.27%    mFD/cost: 449.07%    Origin: 4     Mastery: 0     LotD: 1     Ace: 1     Mark: 0     Rift Break: 0
Cost: 17       FD Gain: 32.49%    mFD/cost: 366.82%    Origin: 7     Mastery: 0     LotD: 1     Ace: 1     Mark: 0     Rift Break: 0
Cost: 20       FD Gain: 33.39%    mFD/cost: 298.11%    Origin: 7     Mastery: 0     LotD: 4     Ace: 1     Mark: 0     Rift Break: 0
Cost: 26       FD Gain: 34.87%    mFD/cost: 245.29%    Origin: 9     Mastery: 0     LotD: 4     Ace: 1     Mark: 0     Rift Break: 0
Cost: 30       FD Gain: 35.76%    mFD/cost: 221.74%    Origin: 9     Mastery: 0     LotD: 4     Ace: 1     Mark: 1     Rift Break: 0
Cost: 70       FD Gain: 42.83%    mFD/cost: 170.94%    Origin: 18    Mastery: 0     LotD: 4     Ace: 1     Mark: 1     Rift Break: 0
Cost: 73       FD Gain: 43.32%    mFD/cost: 164.13%    Origin: 18    Mastery: 0     LotD: 4     Ace: 4     Mark: 1     Rift Break: 0
Cost: 77       FD Gain: 43.98%    mFD/cost: 163.77%    Origin: 18    Mastery: 0     LotD: 4     Ace: 4     Mark: 1     Rift Break: 1
Cost: 97       FD Gain: 47.27%    mFD/cost: 161.93%    Origin: 18    Mastery: 0     LotD: 10    Ace: 4     Mark: 1     Rift Break: 1
Cost: 187      FD Gain: 59.30%    mFD/cost: 126.34%    Origin: 30    Mastery: 0     LotD: 10    Ace: 4     Mark: 1     Rift Break: 1
Cost: 227      FD Gain: 63.79%    mFD/cost: 109.74%    Origin: 30    Mastery: 0     LotD: 20    Ace: 4     Mark: 1     Rift Break: 1
Cost: 236      FD Gain: 64.77%    mFD/cost: 108.24%    Origin: 30    Mastery: 7     LotD: 20    Ace: 4     Mark: 1     Rift Break: 1
Cost: 292      FD Gain: 70.75%    mFD/cost: 103.77%    Origin: 30    Mastery: 7     LotD: 30    Ace: 4     Mark: 1     Rift Break: 1
Cost: 296      FD Gain: 71.13%    mFD/cost:  95.66%    Origin: 30    Mastery: 9     LotD: 30    Ace: 4     Mark: 1     Rift Break: 1
Cost: 316      FD Gain: 72.94%    mFD/cost:  89.65%    Origin: 30    Mastery: 9     LotD: 30    Ace: 10    Mark: 1     Rift Break: 1
Cost: 337      FD Gain: 74.66%    mFD/cost:  81.45%    Origin: 30    Mastery: 18    LotD: 30    Ace: 10    Mark: 1     Rift Break: 1
Cost: 340      FD Gain: 74.91%    mFD/cost:  80.83%    Origin: 30    Mastery: 18    LotD: 30    Ace: 10    Mark: 4     Rift Break: 1
Cost: 343      FD Gain: 75.10%    mFD/cost:  63.83%    Origin: 30    Mastery: 19    LotD: 30    Ace: 10    Mark: 4     Rift Break: 1
Cost: 383      FD Gain: 77.56%    mFD/cost:  60.92%    Origin: 30    Mastery: 19    LotD: 30    Ace: 20    Mark: 4     Rift Break: 1
Cost: 386      FD Gain: 77.74%    mFD/cost:  59.66%    Origin: 30    Mastery: 19    LotD: 30    Ace: 20    Mark: 4     Rift Break: 4
Cost: 442      FD Gain: 81.03%    mFD/cost:  57.79%    Origin: 30    Mastery: 19    LotD: 30    Ace: 30    Mark: 4     Rift Break: 4
Cost: 478      FD Gain: 82.95%    mFD/cost:  52.73%    Origin: 30    Mastery: 29    LotD: 30    Ace: 30    Mark: 4     Rift Break: 4
Cost: 498      FD Gain: 83.84%    mFD/cost:  44.31%    Origin: 30    Mastery: 29    LotD: 30    Ace: 30    Mark: 10    Rift Break: 4
Cost: 518      FD Gain: 84.49%    mFD/cost:  32.73%    Origin: 30    Mastery: 29    LotD: 30    Ace: 30    Mark: 10    Rift Break: 10
Cost: 614      FD Gain: 87.32%    mFD/cost:  29.09%    Origin: 30    Mastery: 29    LotD: 30    Ace: 30    Mark: 30    Rift Break: 10
Cost: 710      FD Gain: 89.41%    mFD/cost:  21.54%    Origin: 30    Mastery: 29    LotD: 30    Ace: 30    Mark: 30    Rift Break: 30
Cost: 720      FD Gain: 89.61%    mFD/cost:  19.14%    Origin: 30    Mastery: 30    LotD: 30    Ace: 30    Mark: 30    Rift Break: 30
```
