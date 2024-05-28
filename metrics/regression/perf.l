;;;  SPDX-FileCopyrightText: Copyright 2017 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; return the elapsed process time in usec
;;; from applying fn to args
;;;
(crux:intern (crux:find-ns "") "perf" (crux:make-ns "perf"))

(crux:intern perf "time-delta"
  (:lambda (fn args)
    ((:lambda (start stop)
       (crux:difference stop start))
       (crux:utime)
       ((:lambda ()
          (crux:apply fn args)
          (crux:utime))))))

;;;
;;; return an vector of storage consumed
;;; by applying fn to args
;;;
(crux:intern perf "storage-delta"
  (:lambda (fn args)
    ((:lambda (start-vec stop-vec)
       (crux:fix
        (:lambda (loop)
          (:if (crux:eq :vector (crux:type-of loop))
               loop
               ((:lambda (list n)
                  ((:lambda (offset)
                     (:if (crux:less-than offset (crux:vector-len start-vec))
                          ((:lambda (type total alloc in-use)
                             ((:lambda (type-stats)
                                (crux:cons `(,@list ,@type-stats) (crux:sum 1 n)))
                              `(,type ,total ,alloc ,in-use)))
                           (crux:vector-ref start-vec (crux:sum offset 0))
                           (crux:difference (crux:vector-ref stop-vec (crux:sum offset 1)) (crux:vector-ref start-vec (crux:sum offset 1)))
                           (crux:difference (crux:vector-ref stop-vec (crux:sum offset 2)) (crux:vector-ref start-vec (crux:sum offset 2)))
                           (crux:difference (crux:vector-ref stop-vec (crux:sum offset 3)) (crux:vector-ref start-vec (crux:sum offset 3))))
                          (crux:make-vector :t list)))
                         (crux:product n 4)))
                 (crux:car loop)
                 (crux:cdr loop))))
         (crux:cons () 1)))
     (crux:heap-stat)
     ((:lambda ()
         (crux:apply fn args)
         (crux:heap-stat))))))
