var searchIndex = JSON.parse('{\
"wave":{"doc":"","t":[2,2,2,2,2,2,2,0,0,0,0,17,17,5,5,5,5,5,5,5,5,5,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,5,5,5,5,5,5,11,11,12,12,12,11,11,11,11,11,11,12,12,12,11,11,13,13,4,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,12,12,4,13,3,13,13,13,17,3,3,4,13,13,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,11,11,12,12,11,11,11,11,11,11,11,11,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["Gesture","GestureRecognizer","HandState","RecognizerParams","RecognizerResult","RecognizerStatus","SensorMeasurement","cbind","math","measurements","recognizer","RES_X","RES_Y","coords_cartesian_from_spherical","coords_spherical_from_cartesian","gesture_recognizer_new","gesture_recognizer_reset","gesture_recognizer_update","recognizer_params_default","recognizer_result_default","sensor_measurement_empty","sensor_params_default_vl53l5cx","CoordsCartesian","CoordsSpherical","borrow","borrow","borrow_mut","borrow_mut","clone","clone","dist_to","eq","eq","fmt","fmt","from","from","from","from","into","into","invalid","is_invalid","matrix_2d_reverse_cols","matrix_2d_reverse_rows","matrix_2d_rotate_180deg","matrix_2d_rotate_270deg","matrix_2d_rotate_90deg","matrix_2d_transpose","partial_cmp","partial_cmp","phi","r","theta","try_from","try_from","try_into","try_into","type_id","type_id","x","y","z","zero","zero","HandFound","HandNotFound","HandState","SensorMeasurement","SensorParams","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","default_vl53l5cx","eq","fmt","fmt","fmt","fov_horizontal","fov_vertical","from","from","from","into","into","into","invalid","new","partial_cmp","time_ms","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","zone_dist","hand_pos","Gesture","GestureNone","GestureRecognizer","GestureStaticHold","GestureSwipeLeft","GestureSwipeRight","HISTORY_SIZE","RecognizerParams","RecognizerResult","RecognizerStatus","RecognizerStatusInitFailure","RecognizerStatusInvalidInput","RecognizerStatusOk","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone","clone","cmp","default","default","eq","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","gesture","gesture_threshold_dist","get_params","get_sensor_params","hand_state","horizontal_swipe_travel_dist","into","into","into","into","into","new","partial_cmp","reset","static_hold_time_ms","static_hold_tolerance_dist","swipe_tolerance_dist","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","update"],"q":["wave","","","","","","","","","","","wave::cbind","","","","","","","","","","","wave::math","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","wave::measurements","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","wave::measurements::HandState","wave::recognizer","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","C bindings","helper math functions to prepare measurements, convert …","measurements coming from the sensor","the main entry point in the library.","Overwrite this define to change the x-axis resolution of …","Overwrite this define to change the y-axis resolution of …","","","","","","","","","","","Represents spherical coordinates in mathematical naming …","","","","","","","","","","","","Returns the argument unchanged.","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","Mirror / reverse the columns of the matrix","Mirror / reverse the rows of the matrix","Rotates the matrix 180 degrees","Rotates the matrix 270 degrees in clockwise direction","Rotates the matrix 90 degrees in clockwise direction","Transposes the given matrix","","","angle with respect to polar axis / z-axis (zenith) (rad)","distance to the origin","angle with respect to x-axis (azimuth) (rad)","","","","","","","","","","","","","","","Represents a sensor measurement coming from the TOF sensor.","Configurable sensor parameters. Different for every sensor.","","","","","","","","","","The default parameters for the ST VL53L5CX TOF-Sensor.","","","","","The horizontal FOV of the sensor","The verctical FOV of the sensor","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","An invalid measurement, meaning all values are set to …","Creates a new measurement","","The time in ms of the measurement. Must be monotonically …","","","","","","","","","","the measured distances of each zone.","","Represents a hand gesture","No recognized gesture","The gesture recognizer. Is initially configured through …","A static hold, meaning the hand is hovering still for a …","A left swipe","A right swipe","DO NOT MODIFY","Parameters for gesture recognition","A gesture prediction result.","The status of the gesture recognizer","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","The recognized gesture, GestureNone if no gesture was …","The furthest hand distance for gesture recognition","Gets the current configured gesture recognizer parameters","Gets the current configured sensor parameters","The current hand state","How much the hand has to move in one distance while doing …","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","A new gesture recognizer initialized with default …","","Resets the gesture recognizer with the given parameters. …","The time the hand has to be still to recognize a static …","How much the hand can move from and towards the sensor …","How much the hand can move from and towards the sensor …","","","","","","","","","","","","","","","","Updates the gesture recognizer with a new measurement and …"],"i":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,1,2,1,2,1,2,2,1,2,1,2,2,1,1,2,1,1,1,0,0,0,0,0,0,2,1,1,1,1,2,1,2,1,2,1,2,2,2,2,1,18,18,0,0,0,18,3,8,18,3,8,18,3,8,3,8,18,3,8,3,3,18,3,8,18,3,8,8,8,8,8,18,3,8,18,3,8,18,3,8,8,20,0,19,0,19,19,19,0,0,0,0,7,7,7,19,9,5,7,4,19,9,5,7,4,19,9,5,7,4,19,9,5,19,19,9,5,7,4,19,9,5,7,4,9,5,4,4,9,5,19,9,5,7,4,4,19,4,5,5,5,19,9,5,7,4,19,9,5,7,4,19,9,5,7,4,4],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,[1,2],[2,1],[3,4],[[4,5,3,6],7],[[4,8,9],7],[[],5],[[],9],[[],8],[[],3],0,0,[[]],[[]],[[]],[[]],[2,2],[1,1],[[2,2],10],[[2,2],11],[[1,1],11],[[2,12],13],[[1,12],13],[[]],[1,2],[2,1],[[]],[[]],[[]],[[],1],[1,11],[[]],[[]],[[]],[[]],[[]],[[]],[[2,2],[[15,[14]]]],[[1,1],[[15,[14]]]],0,0,0,[[],16],[[],16],[[],16],[[],16],[[],17],[[],17],0,0,0,[[],2],[[],1],0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[18,18],[3,3],[8,8],[[],3],[[8,8],11],[[18,12],13],[[3,12],13],[[8,12],13],0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[],8],[[],8],[[8,8],[[15,[14]]]],0,[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],17],[[],17],[[],17],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[19,19],[9,9],[5,5],[7,7],[4,4],[[19,19],14],[[],9],[[],5],[[19,19],11],[[19,12],13],[[9,12],13],[[5,12],13],[[7,12],13],[[4,12],13],[[]],[[]],[[]],[[]],[[]],0,0,[4,5],[4,3],0,0,[[]],[[]],[[]],[[]],[[]],[3,4],[[19,19],[[15,[14]]]],[[4,5,3,6],7],0,0,0,[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],17],[[],17],[[],17],[[],17],[[],17],[[4,8,9],7]],"p":[[3,"CoordsSpherical"],[3,"CoordsCartesian"],[3,"SensorParams"],[3,"GestureRecognizer"],[3,"RecognizerParams"],[15,"u32"],[4,"RecognizerStatus"],[3,"SensorMeasurement"],[3,"RecognizerResult"],[15,"f32"],[15,"bool"],[3,"Formatter"],[6,"Result"],[4,"Ordering"],[4,"Option"],[4,"Result"],[3,"TypeId"],[4,"HandState"],[4,"Gesture"],[13,"HandFound"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
