class Data:
    """An example data point

    Args:
        x = our datum
    """

    def __init__(self, x=1):
        self.x = x 

    def __repr__(self):
        return f"Data(x={self.x})"

    def get_x(self):
        """Get the value of x"""
        return self.x
