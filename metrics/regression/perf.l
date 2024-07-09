;;;  SPDX-FileCopyrightText: Copyright 2017 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; return the elapsed process time in usec
;;; from applying fn to args
;;;
(mu:intern (mu:find-ns "") "perf" (mu:make-ns "perf"))

(mu:intern perf "time-delta"
  (:lambda (fn args)
    ((:lambda (start stop)
       (mu:difference stop start))
       (mu:utime)
       ((:lambda ()
          (mu:apply fn args)
          (mu:utime))))))

;;;
;;; return an vector of storage consumed
;;; by applying fn to args
;;;
(mu:intern perf "storage-delta"
  (:lambda (fn args)
    ((:lambda (start-vec stop-vec)
       (mu:fix
        (:lambda (loop)
          (:if (mu:eq :vector (mu:type-of loop))
               loop
               ((:lambda (list n)
                  ((:lambda (offset)
                     (:if (mu:less-than offset (mu:vector-len start-vec))
                          ((:lambda (type total alloc in-use)
                             ((:lambda (type-stats)
                                (mu:cons `(,@list ,@type-stats) (mu:sum 1 n)))
                              `(,type ,total ,alloc ,in-use)))
                           (mu:svref start-vec (mu:sum offset 0))
                           (mu:difference (mu:svref stop-vec (mu:sum offset 1)) (mu:svref start-vec (mu:sum offset 1)))
                           (mu:difference (mu:svref stop-vec (mu:sum offset 2)) (mu:svref start-vec (mu:sum offset 2)))
                           (mu:difference (mu:svref stop-vec (mu:sum offset 3)) (mu:svref start-vec (mu:sum offset 3))))
                          (mu:make-vector :t list)))
                         (mu:product n 4)))
                 (mu:car loop)
                 (mu:cdr loop))))
         (mu:cons () 1)))
     (mu:heap-stat)
     ((:lambda ()
         (mu:apply fn args)
         (mu:heap-stat))))))
