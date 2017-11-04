getSqrt n = round $ sqrt $ fromIntegral n
factors n = filter(\x -> n `rem` x == 0)[2..(getSqrt n)]
primes n = null $ factors n

main = do
       let num = 600581475143
       let primeFactors = filter primes $ factors num
       print(last primeFactors)

