class Group(object):

    @staticmethod
    def get_multiplicative_group(n):
        z_star = (x for x in self.Z if np.gcd(x, n) == 1)
        return z_star
    