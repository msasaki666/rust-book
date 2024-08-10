price = 3950

for i500 in range(0, 11):
  for i100 in range(0, 4):
    for i50 in range(0, 11):
      total = 500 * i500 + 100 * i100 + 50 * i50
      if price == total:
        print(f'500円×{i500}+100円×{i100}+50円×{i50}={total}')
