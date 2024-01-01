BACK_DIR="./back"
FRONT_DIR="./front/pi-status-front/"

default:
	make -C $(BACK_DIR) default
	make -C $(FRONT_DIR) default

arm:
	make -C $(BACK_DIR) arm
	make -C $(FRONT_DIR) arm

clean:
	make -C $(BACK_DIR) clean
	make -C $(FRONT_DIR) clean
