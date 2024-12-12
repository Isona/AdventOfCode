import mlrose

dist_list = []

with open("9testinput.txt") as file:
    for line in file:
        value = line.split()
        dist_list.append((int(value[0]),int(value[2]),int(value[4])))

print(dist_list)


fitness_dists = mlrose.TravellingSales(distances = dist_list)
problem_fit = mlrose.TSPOpt(length = 3, fitness_fn = fitness_dists,
                            maximize=True)

best_state, best_fitness = mlrose.genetic_alg(problem_fit, max_attempts = 100, random_state = 2)

print('The best state found is: ', best_state)
print('The fitness at the best state is: ', best_fitness)
